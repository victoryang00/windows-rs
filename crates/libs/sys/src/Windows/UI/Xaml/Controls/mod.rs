#[cfg(feature = "UI_Xaml_Controls_Maps")]
pub mod Maps;
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
pub mod Primitives;
pub type AnchorRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AppBar = *mut ::core::ffi::c_void;
pub type AppBarButton = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct AppBarClosedDisplayMode(pub i32);
impl AppBarClosedDisplayMode {
    pub const Compact: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBarClosedDisplayMode {}
impl ::core::clone::Clone for AppBarClosedDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBarElementContainer = *mut ::core::ffi::c_void;
pub type AppBarSeparator = *mut ::core::ffi::c_void;
pub type AppBarToggleButton = *mut ::core::ffi::c_void;
pub type AutoSuggestBox = *mut ::core::ffi::c_void;
pub type AutoSuggestBoxQuerySubmittedEventArgs = *mut ::core::ffi::c_void;
pub type AutoSuggestBoxSuggestionChosenEventArgs = *mut ::core::ffi::c_void;
pub type AutoSuggestBoxTextChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct AutoSuggestionBoxTextChangeReason(pub i32);
impl AutoSuggestionBoxTextChangeReason {
    pub const UserInput: Self = Self(0i32);
    pub const ProgrammaticChange: Self = Self(1i32);
    pub const SuggestionChosen: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoSuggestionBoxTextChangeReason {}
impl ::core::clone::Clone for AutoSuggestionBoxTextChangeReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackClickEventArgs = *mut ::core::ffi::c_void;
pub type BackClickEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct BackgroundSizing(pub i32);
impl BackgroundSizing {
    pub const InnerBorderEdge: Self = Self(0i32);
    pub const OuterBorderEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundSizing {}
impl ::core::clone::Clone for BackgroundSizing {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BitmapIcon = *mut ::core::ffi::c_void;
pub type BitmapIconSource = *mut ::core::ffi::c_void;
pub type Border = *mut ::core::ffi::c_void;
pub type Button = *mut ::core::ffi::c_void;
pub type CalendarDatePicker = *mut ::core::ffi::c_void;
pub type CalendarDatePickerDateChangedEventArgs = *mut ::core::ffi::c_void;
pub type CalendarView = *mut ::core::ffi::c_void;
pub type CalendarViewDayItem = *mut ::core::ffi::c_void;
pub type CalendarViewDayItemChangingEventArgs = *mut ::core::ffi::c_void;
pub type CalendarViewDayItemChangingEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CalendarViewDisplayMode(pub i32);
impl CalendarViewDisplayMode {
    pub const Month: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Decade: Self = Self(2i32);
}
impl ::core::marker::Copy for CalendarViewDisplayMode {}
impl ::core::clone::Clone for CalendarViewDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CalendarViewSelectedDatesChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CalendarViewSelectionMode(pub i32);
impl CalendarViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for CalendarViewSelectionMode {}
impl ::core::clone::Clone for CalendarViewSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CandidateWindowAlignment(pub i32);
impl CandidateWindowAlignment {
    pub const Default: Self = Self(0i32);
    pub const BottomEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for CandidateWindowAlignment {}
impl ::core::clone::Clone for CandidateWindowAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CandidateWindowBoundsChangedEventArgs = *mut ::core::ffi::c_void;
pub type Canvas = *mut ::core::ffi::c_void;
pub type CaptureElement = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CharacterCasing(pub i32);
impl CharacterCasing {
    pub const Normal: Self = Self(0i32);
    pub const Lower: Self = Self(1i32);
    pub const Upper: Self = Self(2i32);
}
impl ::core::marker::Copy for CharacterCasing {}
impl ::core::clone::Clone for CharacterCasing {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CheckBox = *mut ::core::ffi::c_void;
pub type ChoosingGroupHeaderContainerEventArgs = *mut ::core::ffi::c_void;
pub type ChoosingItemContainerEventArgs = *mut ::core::ffi::c_void;
pub type CleanUpVirtualizedItemEventArgs = *mut ::core::ffi::c_void;
pub type CleanUpVirtualizedItemEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ClickMode(pub i32);
impl ClickMode {
    pub const Release: Self = Self(0i32);
    pub const Press: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
}
impl ::core::marker::Copy for ClickMode {}
impl ::core::clone::Clone for ClickMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ColorChangedEventArgs = *mut ::core::ffi::c_void;
pub type ColorPicker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ColorPickerHsvChannel(pub i32);
impl ColorPickerHsvChannel {
    pub const Hue: Self = Self(0i32);
    pub const Saturation: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
    pub const Alpha: Self = Self(3i32);
}
impl ::core::marker::Copy for ColorPickerHsvChannel {}
impl ::core::clone::Clone for ColorPickerHsvChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ColorSpectrumComponents(pub i32);
impl ColorSpectrumComponents {
    pub const HueValue: Self = Self(0i32);
    pub const ValueHue: Self = Self(1i32);
    pub const HueSaturation: Self = Self(2i32);
    pub const SaturationHue: Self = Self(3i32);
    pub const SaturationValue: Self = Self(4i32);
    pub const ValueSaturation: Self = Self(5i32);
}
impl ::core::marker::Copy for ColorSpectrumComponents {}
impl ::core::clone::Clone for ColorSpectrumComponents {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ColorSpectrumShape(pub i32);
impl ColorSpectrumShape {
    pub const Box: Self = Self(0i32);
    pub const Ring: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorSpectrumShape {}
impl ::core::clone::Clone for ColorSpectrumShape {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ColumnDefinition = *mut ::core::ffi::c_void;
pub type ColumnDefinitionCollection = *mut ::core::ffi::c_void;
pub type ComboBox = *mut ::core::ffi::c_void;
pub type ComboBoxItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ComboBoxSelectionChangedTrigger(pub i32);
impl ComboBoxSelectionChangedTrigger {
    pub const Committed: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl ::core::marker::Copy for ComboBoxSelectionChangedTrigger {}
impl ::core::clone::Clone for ComboBoxSelectionChangedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ComboBoxTextSubmittedEventArgs = *mut ::core::ffi::c_void;
pub type CommandBar = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CommandBarDefaultLabelPosition(pub i32);
impl CommandBarDefaultLabelPosition {
    pub const Bottom: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Collapsed: Self = Self(2i32);
}
impl ::core::marker::Copy for CommandBarDefaultLabelPosition {}
impl ::core::clone::Clone for CommandBarDefaultLabelPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CommandBarDynamicOverflowAction(pub i32);
impl CommandBarDynamicOverflowAction {
    pub const AddingToOverflow: Self = Self(0i32);
    pub const RemovingFromOverflow: Self = Self(1i32);
}
impl ::core::marker::Copy for CommandBarDynamicOverflowAction {}
impl ::core::clone::Clone for CommandBarDynamicOverflowAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CommandBarFlyout = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CommandBarLabelPosition(pub i32);
impl CommandBarLabelPosition {
    pub const Default: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
}
impl ::core::marker::Copy for CommandBarLabelPosition {}
impl ::core::clone::Clone for CommandBarLabelPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct CommandBarOverflowButtonVisibility(pub i32);
impl CommandBarOverflowButtonVisibility {
    pub const Auto: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Collapsed: Self = Self(2i32);
}
impl ::core::marker::Copy for CommandBarOverflowButtonVisibility {}
impl ::core::clone::Clone for CommandBarOverflowButtonVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CommandBarOverflowPresenter = *mut ::core::ffi::c_void;
pub type ContainerContentChangingEventArgs = *mut ::core::ffi::c_void;
pub type ContentControl = *mut ::core::ffi::c_void;
pub type ContentDialog = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ContentDialogButton(pub i32);
impl ContentDialogButton {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Secondary: Self = Self(2i32);
    pub const Close: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentDialogButton {}
impl ::core::clone::Clone for ContentDialogButton {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContentDialogButtonClickDeferral = *mut ::core::ffi::c_void;
pub type ContentDialogButtonClickEventArgs = *mut ::core::ffi::c_void;
pub type ContentDialogClosedEventArgs = *mut ::core::ffi::c_void;
pub type ContentDialogClosingDeferral = *mut ::core::ffi::c_void;
pub type ContentDialogClosingEventArgs = *mut ::core::ffi::c_void;
pub type ContentDialogOpenedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ContentDialogPlacement(pub i32);
impl ContentDialogPlacement {
    pub const Popup: Self = Self(0i32);
    pub const InPlace: Self = Self(1i32);
}
impl ::core::marker::Copy for ContentDialogPlacement {}
impl ::core::clone::Clone for ContentDialogPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ContentDialogResult(pub i32);
impl ContentDialogResult {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Secondary: Self = Self(2i32);
}
impl ::core::marker::Copy for ContentDialogResult {}
impl ::core::clone::Clone for ContentDialogResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ContentLinkChangeKind(pub i32);
impl ContentLinkChangeKind {
    pub const Inserted: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Edited: Self = Self(2i32);
}
impl ::core::marker::Copy for ContentLinkChangeKind {}
impl ::core::clone::Clone for ContentLinkChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContentLinkChangedEventArgs = *mut ::core::ffi::c_void;
pub type ContentPresenter = *mut ::core::ffi::c_void;
pub type ContextMenuEventArgs = *mut ::core::ffi::c_void;
pub type ContextMenuOpeningEventHandler = *mut ::core::ffi::c_void;
pub type Control = *mut ::core::ffi::c_void;
pub type ControlTemplate = *mut ::core::ffi::c_void;
pub type DataTemplateSelector = *mut ::core::ffi::c_void;
pub type DatePickedEventArgs = *mut ::core::ffi::c_void;
pub type DatePicker = *mut ::core::ffi::c_void;
pub type DatePickerFlyout = *mut ::core::ffi::c_void;
pub type DatePickerFlyoutItem = *mut ::core::ffi::c_void;
pub type DatePickerFlyoutPresenter = *mut ::core::ffi::c_void;
pub type DatePickerSelectedValueChangedEventArgs = *mut ::core::ffi::c_void;
pub type DatePickerValueChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct DisabledFormattingAccelerators(pub u32);
impl DisabledFormattingAccelerators {
    pub const None: Self = Self(0u32);
    pub const Bold: Self = Self(1u32);
    pub const Italic: Self = Self(2u32);
    pub const Underline: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for DisabledFormattingAccelerators {}
impl ::core::clone::Clone for DisabledFormattingAccelerators {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DragItemsCompletedEventArgs = *mut ::core::ffi::c_void;
pub type DragItemsStartingEventArgs = *mut ::core::ffi::c_void;
pub type DragItemsStartingEventHandler = *mut ::core::ffi::c_void;
pub type DropDownButton = *mut ::core::ffi::c_void;
pub type DropDownButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type DynamicOverflowItemsChangingEventArgs = *mut ::core::ffi::c_void;
pub type FlipView = *mut ::core::ffi::c_void;
pub type FlipViewItem = *mut ::core::ffi::c_void;
pub type Flyout = *mut ::core::ffi::c_void;
pub type FlyoutPresenter = *mut ::core::ffi::c_void;
pub type FocusDisengagedEventArgs = *mut ::core::ffi::c_void;
pub type FocusEngagedEventArgs = *mut ::core::ffi::c_void;
pub type FontIcon = *mut ::core::ffi::c_void;
pub type FontIconSource = *mut ::core::ffi::c_void;
pub type Frame = *mut ::core::ffi::c_void;
pub type Grid = *mut ::core::ffi::c_void;
pub type GridView = *mut ::core::ffi::c_void;
pub type GridViewHeaderItem = *mut ::core::ffi::c_void;
pub type GridViewItem = *mut ::core::ffi::c_void;
pub type GroupItem = *mut ::core::ffi::c_void;
pub type GroupStyle = *mut ::core::ffi::c_void;
pub type GroupStyleSelector = *mut ::core::ffi::c_void;
pub type HandwritingPanelClosedEventArgs = *mut ::core::ffi::c_void;
pub type HandwritingPanelOpenedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct HandwritingPanelPlacementAlignment(pub i32);
impl HandwritingPanelPlacementAlignment {
    pub const Auto: Self = Self(0i32);
    pub const TopLeft: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const BottomLeft: Self = Self(3i32);
    pub const BottomRight: Self = Self(4i32);
}
impl ::core::marker::Copy for HandwritingPanelPlacementAlignment {}
impl ::core::clone::Clone for HandwritingPanelPlacementAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HandwritingView = *mut ::core::ffi::c_void;
pub type HandwritingViewCandidatesChangedEventArgs = *mut ::core::ffi::c_void;
pub type HandwritingViewTextSubmittedEventArgs = *mut ::core::ffi::c_void;
pub type Hub = *mut ::core::ffi::c_void;
pub type HubSection = *mut ::core::ffi::c_void;
pub type HubSectionCollection = *mut ::core::ffi::c_void;
pub type HubSectionHeaderClickEventArgs = *mut ::core::ffi::c_void;
pub type HubSectionHeaderClickEventHandler = *mut ::core::ffi::c_void;
pub type HyperlinkButton = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAnchorRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Anchor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAnchor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AnchorCandidates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AnchorCandidates: usize,
}
#[repr(C)]
pub struct IAppBar {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSticky: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSticky: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[repr(C)]
pub struct IAppBar2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClosedDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBarClosedDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetClosedDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppBarClosedDisplayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBar3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "Foundation")]
    pub Opening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpening: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
#[repr(C)]
pub struct IAppBar4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButton3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CommandBarLabelPosition) -> ::windows_sys::core::HRESULT,
    pub SetLabelPosition: unsafe extern "system" fn(this: *mut *mut Self, value: CommandBarLabelPosition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButton4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetKeyboardAcceleratorTextOverride: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButton5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IAppBarButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCompactProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButtonStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelPositionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsInOverflowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DynamicOverflowOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarButtonStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextOverrideProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarElementContainer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppBarElementContainerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarElementContainerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCompactProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsInOverflowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DynamicOverflowOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnClosed: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnOpened: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarOverrides3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnClosing: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnOpening: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarSeparator {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppBarSeparatorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarSeparatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCompactProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarSeparatorStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInOverflowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DynamicOverflowOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStickyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClosedDisplayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButton3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CommandBarLabelPosition) -> ::windows_sys::core::HRESULT,
    pub SetLabelPosition: unsafe extern "system" fn(this: *mut *mut Self, value: CommandBarLabelPosition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButton4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetKeyboardAcceleratorTextOverride: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButton5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IAppBarToggleButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCompactProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButtonStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelPositionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsInOverflowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DynamicOverflowOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButtonStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextOverrideProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxSuggestionListHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxSuggestionListHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsSuggestionListOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSuggestionListOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TextMemberPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTextMemberPath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UpdateTextOnSelect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUpdateTextOnSelect: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoMaximizeSuggestionArea: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoMaximizeSuggestionArea: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TextBoxStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTextBoxStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestionChosen: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestionChosen: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuggestionChosen: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuggestionChosen: usize,
    #[cfg(feature = "Foundation")]
    pub TextChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextChanged: usize,
}
#[repr(C)]
pub struct IAutoSuggestBox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryIcon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetQueryIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub QuerySubmitted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuerySubmitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuerySubmitted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuerySubmitted: usize,
}
#[repr(C)]
pub struct IAutoSuggestBox3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBox4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxQuerySubmittedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ChosenSuggestion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxSuggestionListHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSuggestionListOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextMemberPathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateTextOnSelectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoMaximizeSuggestionAreaProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextBoxStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryIconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxSuggestionChosenEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxTextChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutoSuggestionBoxTextChangeReason) -> ::windows_sys::core::HRESULT,
    pub SetReason: unsafe extern "system" fn(this: *mut *mut Self, value: AutoSuggestionBoxTextChangeReason) -> ::windows_sys::core::HRESULT,
    pub CheckCurrent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoSuggestBoxTextChangedEventArgsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReasonProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIcon {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UriSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriSource: usize,
    #[cfg(feature = "Foundation")]
    pub SetUriSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUriSource: usize,
}
#[repr(C)]
pub struct IBitmapIcon2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowAsMonochrome: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowAsMonochrome: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIconSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UriSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriSource: usize,
    #[cfg(feature = "Foundation")]
    pub SetUriSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUriSource: usize,
    pub ShowAsMonochrome: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowAsMonochrome: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIconSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIconSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UriSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowAsMonochromeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIconStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UriSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapIconStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowAsMonochromeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBorder {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub Child: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ChildTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ChildTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetChildTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetChildTransitions: usize,
}
#[repr(C)]
pub struct IBorder2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub BackgroundTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBorderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBorderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IButtonStaticsWithFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IButtonWithFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Flyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Flyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetFlyout: usize,
}
#[repr(C)]
pub struct ICalendarDatePicker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    pub IsCalendarOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCalendarOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DateFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDateFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarViewStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCalendarViewStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MinDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinDate: usize,
    #[cfg(feature = "Foundation")]
    pub MaxDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxDate: usize,
    pub IsTodayHighlighted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTodayHighlighted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CalendarViewDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, value: CalendarViewDisplayMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub FirstDayOfWeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    FirstDayOfWeek: usize,
    #[cfg(feature = "Globalization")]
    pub SetFirstDayOfWeek: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SetFirstDayOfWeek: usize,
    pub DayOfWeekFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDayOfWeekFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOutOfScopeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOutOfScopeEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsGroupLabelVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGroupLabelVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CalendarViewDayItemChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CalendarViewDayItemChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCalendarViewDayItemChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCalendarViewDayItemChanging: usize,
    #[cfg(feature = "Foundation")]
    pub DateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayDate: unsafe extern "system" fn(this: *mut *mut Self, date: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayDate: usize,
    pub SetYearDecadeDisplayDimensions: unsafe extern "system" fn(this: *mut *mut Self, columns: i32, rows: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarDatePicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarDatePicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarDatePickerDateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NewDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDate: usize,
    #[cfg(feature = "Foundation")]
    pub OldDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldDate: usize,
}
#[repr(C)]
pub struct ICalendarDatePickerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarDatePickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCalendarOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DateFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarViewStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinDateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxDateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTodayHighlightedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstDayOfWeekProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayOfWeekFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarIdentifierProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOutOfScopeEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsGroupLabelVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarDatePickerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarDatePickerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarView {
    pub base__: ::windows_sys::core::IInspectable,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DayOfWeekFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDayOfWeekFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsGroupLabelVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGroupLabelVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CalendarViewDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, value: CalendarViewDisplayMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub FirstDayOfWeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    FirstDayOfWeek: usize,
    #[cfg(feature = "Globalization")]
    pub SetFirstDayOfWeek: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SetFirstDayOfWeek: usize,
    pub IsOutOfScopeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOutOfScopeEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsTodayHighlighted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTodayHighlighted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxDate: usize,
    #[cfg(feature = "Foundation")]
    pub MinDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinDate: usize,
    pub NumberOfWeeksInView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetNumberOfWeeksInView: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectedDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectedDates: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CalendarViewSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: CalendarViewSelectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedHoverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedHoverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedHoverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedHoverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HoverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HoverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetHoverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetHoverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CalendarItemBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CalendarItemBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCalendarItemBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCalendarItemBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub OutOfScopeBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    OutOfScopeBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetOutOfScopeBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetOutOfScopeBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CalendarItemBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CalendarItemBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCalendarItemBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCalendarItemBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PressedForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PressedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPressedForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPressedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BlackoutForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BlackoutForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBlackoutForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBlackoutForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub OutOfScopeForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    OutOfScopeForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetOutOfScopeForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetOutOfScopeForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CalendarItemForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CalendarItemForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCalendarItemForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCalendarItemForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DayItemFontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DayItemFontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDayItemFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDayItemFontFamily: usize,
    pub DayItemFontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDayItemFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub DayItemFontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    DayItemFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetDayItemFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetDayItemFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub DayItemFontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    DayItemFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetDayItemFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetDayItemFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub TodayFontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    TodayFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetTodayFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetTodayFontWeight: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FirstOfMonthLabelFontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FirstOfMonthLabelFontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFirstOfMonthLabelFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFirstOfMonthLabelFontFamily: usize,
    pub FirstOfMonthLabelFontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFirstOfMonthLabelFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FirstOfMonthLabelFontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FirstOfMonthLabelFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFirstOfMonthLabelFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFirstOfMonthLabelFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FirstOfMonthLabelFontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FirstOfMonthLabelFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFirstOfMonthLabelFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFirstOfMonthLabelFontWeight: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub MonthYearItemFontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    MonthYearItemFontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetMonthYearItemFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetMonthYearItemFontFamily: usize,
    pub MonthYearItemFontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMonthYearItemFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub MonthYearItemFontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    MonthYearItemFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetMonthYearItemFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetMonthYearItemFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub MonthYearItemFontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    MonthYearItemFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetMonthYearItemFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetMonthYearItemFontWeight: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FirstOfYearDecadeLabelFontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FirstOfYearDecadeLabelFontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFirstOfYearDecadeLabelFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFirstOfYearDecadeLabelFontFamily: usize,
    pub FirstOfYearDecadeLabelFontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFirstOfYearDecadeLabelFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FirstOfYearDecadeLabelFontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FirstOfYearDecadeLabelFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFirstOfYearDecadeLabelFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFirstOfYearDecadeLabelFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FirstOfYearDecadeLabelFontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FirstOfYearDecadeLabelFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFirstOfYearDecadeLabelFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFirstOfYearDecadeLabelFontWeight: usize,
    pub HorizontalDayItemAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalDayItemAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalDayItemAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalDayItemAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub HorizontalFirstOfMonthLabelAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalFirstOfMonthLabelAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalFirstOfMonthLabelAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalFirstOfMonthLabelAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub CalendarItemBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetCalendarItemBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub CalendarViewDayItemStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCalendarViewDayItemStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CalendarViewDayItemChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CalendarViewDayItemChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCalendarViewDayItemChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCalendarViewDayItemChanging: usize,
    #[cfg(feature = "Foundation")]
    pub SelectedDatesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedDatesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectedDatesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectedDatesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayDate: unsafe extern "system" fn(this: *mut *mut Self, date: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayDate: usize,
    pub SetYearDecadeDisplayDimensions: unsafe extern "system" fn(this: *mut *mut Self, columns: i32, rows: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarView2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedDisabledBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedDisabledBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodaySelectedInnerBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodaySelectedInnerBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodaySelectedInnerBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodaySelectedInnerBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BlackoutStrikethroughBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BlackoutStrikethroughBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBlackoutStrikethroughBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBlackoutStrikethroughBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BlackoutBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BlackoutBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBlackoutBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBlackoutBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CalendarItemHoverBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CalendarItemHoverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCalendarItemHoverBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCalendarItemHoverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CalendarItemPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CalendarItemPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCalendarItemPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCalendarItemPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CalendarItemDisabledBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CalendarItemDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCalendarItemDisabledBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCalendarItemDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayBlackoutBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayBlackoutBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayBlackoutBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayBlackoutBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayHoverBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayHoverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayHoverBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayHoverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayDisabledBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayDisabledBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TodayBlackoutForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TodayBlackoutForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetTodayBlackoutForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetTodayBlackoutForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedHoverForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedHoverForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedHoverForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedHoverForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPressedForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPressedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPressedForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPressedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedDisabledForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedDisabledForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedDisabledForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedDisabledForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub OutOfScopeHoverForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    OutOfScopeHoverForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetOutOfScopeHoverForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetOutOfScopeHoverForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub OutOfScopePressedForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    OutOfScopePressedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetOutOfScopePressedForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetOutOfScopePressedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DisabledForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DisabledForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDisabledForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDisabledForeground: usize,
    pub DayItemMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetDayItemMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub MonthYearItemMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetMonthYearItemMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub FirstOfMonthLabelMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetFirstOfMonthLabelMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub FirstOfYearDecadeLabelMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetFirstOfYearDecadeLabelMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub CalendarItemCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCalendarItemCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarViewDayItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBlackout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBlackout: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDensityColors: unsafe extern "system" fn(this: *mut *mut Self, colors: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDensityColors: usize,
}
#[repr(C)]
pub struct ICalendarViewDayItemChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InRecycleQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Phase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RegisterUpdateCallback: unsafe extern "system" fn(this: *mut *mut Self, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterUpdateCallback: usize,
    #[cfg(feature = "Foundation")]
    pub RegisterUpdateCallbackWithPhase: unsafe extern "system" fn(this: *mut *mut Self, callbackphase: u32, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterUpdateCallbackWithPhase: usize,
}
#[repr(C)]
pub struct ICalendarViewDayItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarViewDayItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBlackoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarViewSelectedDatesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedDates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedDates: usize,
}
#[repr(C)]
pub struct ICalendarViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CalendarIdentifierProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayOfWeekFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsGroupLabelVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstDayOfWeekProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOutOfScopeEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTodayHighlightedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxDateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinDateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NumberOfWeeksInViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedDatesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TemplateSettingsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedHoverBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPressedBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HoverBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PressedBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutOfScopeBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PressedForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BlackoutForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutOfScopeForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayItemFontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayItemFontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayItemFontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayItemFontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayFontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfMonthLabelFontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfMonthLabelFontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfMonthLabelFontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfMonthLabelFontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthYearItemFontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthYearItemFontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthYearItemFontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthYearItemFontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfYearDecadeLabelFontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfYearDecadeLabelFontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfYearDecadeLabelFontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfYearDecadeLabelFontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalDayItemAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalDayItemAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalFirstOfMonthLabelAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalFirstOfMonthLabelAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemBorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarViewDayItemStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedDisabledBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodaySelectedInnerBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BlackoutStrikethroughBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BlackoutBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemHoverBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemPressedBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemDisabledBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayBlackoutBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayHoverBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayPressedBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayDisabledBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TodayBlackoutForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedHoverForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPressedForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedDisabledForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutOfScopeHoverForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutOfScopePressedForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisabledForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayItemMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthYearItemMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfMonthLabelMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstOfYearDecadeLabelMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarItemCornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICandidateWindowBoundsChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
}
#[repr(C)]
pub struct ICanvas {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICanvasFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICanvasStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLeft: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, length: f64) -> ::windows_sys::core::HRESULT,
    pub TopProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTop: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, length: f64) -> ::windows_sys::core::HRESULT,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetZIndex: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICaptureElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Capture")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    Source: usize,
    #[cfg(feature = "Media_Capture")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    SetSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
}
#[repr(C)]
pub struct ICaptureElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICheckBox {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICheckBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChoosingGroupHeaderContainerEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupHeaderContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGroupHeaderContainer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GroupIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChoosingItemContainerEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ItemContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ItemContainer: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetItemContainer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetItemContainer: usize,
    pub IsContainerPrepared: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsContainerPrepared: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICleanUpVirtualizedItemEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UIElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub NewColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorPicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PreviousColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreviousColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreviousColor: usize,
    pub IsAlphaEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAlphaEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsColorSpectrumVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorSpectrumVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsColorPreviewVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorPreviewVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsColorSliderVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorSliderVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAlphaSliderVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAlphaSliderVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMoreButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMoreButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsColorChannelTextInputVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorChannelTextInputVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAlphaTextInputVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAlphaTextInputVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsHexInputVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHexInputVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
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
    pub ColorSpectrumShape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ColorSpectrumShape) -> ::windows_sys::core::HRESULT,
    pub SetColorSpectrumShape: unsafe extern "system" fn(this: *mut *mut Self, value: ColorSpectrumShape) -> ::windows_sys::core::HRESULT,
    pub ColorSpectrumComponents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ColorSpectrumComponents) -> ::windows_sys::core::HRESULT,
    pub SetColorSpectrumComponents: unsafe extern "system" fn(this: *mut *mut Self, value: ColorSpectrumComponents) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ColorChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ColorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveColorChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveColorChanged: usize,
}
#[repr(C)]
pub struct IColorPickerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreviousColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAlphaEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorSpectrumVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorPreviewVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorSliderVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAlphaSliderVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsMoreButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorChannelTextInputVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAlphaTextInputVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHexInputVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinHueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxHueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinSaturationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxSaturationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColorSpectrumShapeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColorSpectrumComponentsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColumnDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::GridLength) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, value: super::GridLength) -> ::windows_sys::core::HRESULT,
    pub MaxWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ActualWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColumnDefinitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub WidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDropDownOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDropDownOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEditable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSelectionBoxHighlighted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxDropDownHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxDropDownHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SelectionBoxItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionBoxItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "Foundation")]
    pub DropDownClosed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropDownClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDropDownClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDropDownClosed: usize,
    #[cfg(feature = "Foundation")]
    pub DropDownOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropDownOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDropDownOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDropDownOpened: usize,
}
#[repr(C)]
pub struct IComboBox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBox3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub IsTextSearchEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextSearchEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBox4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionChangedTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ComboBoxSelectionChangedTrigger) -> ::windows_sys::core::HRESULT,
    pub SetSelectionChangedTrigger: unsafe extern "system" fn(this: *mut *mut Self, value: ComboBoxSelectionChangedTrigger) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBox5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderForeground: usize,
}
#[repr(C)]
pub struct IComboBox6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsEditable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TextBoxStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTextBoxStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TextSubmitted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextSubmitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextSubmitted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextSubmitted: usize,
}
#[repr(C)]
pub struct IComboBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IComboBoxItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnDropDownClosed: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDropDownOpened: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDropDownOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxDropDownHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextSearchEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionChangedTriggerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaceholderForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEditableProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextBoxStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxTextSubmittedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PrimaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrimaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SecondaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SecondaryCommands: usize,
}
#[repr(C)]
pub struct ICommandBar2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CommandBarOverflowPresenterStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCommandBarOverflowPresenterStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CommandBarTemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CommandBarTemplateSettings: usize,
}
#[repr(C)]
pub struct ICommandBar3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultLabelPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CommandBarDefaultLabelPosition) -> ::windows_sys::core::HRESULT,
    pub SetDefaultLabelPosition: unsafe extern "system" fn(this: *mut *mut Self, value: CommandBarDefaultLabelPosition) -> ::windows_sys::core::HRESULT,
    pub OverflowButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CommandBarOverflowButtonVisibility) -> ::windows_sys::core::HRESULT,
    pub SetOverflowButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, value: CommandBarOverflowButtonVisibility) -> ::windows_sys::core::HRESULT,
    pub IsDynamicOverflowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDynamicOverflowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DynamicOverflowItemsChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DynamicOverflowItemsChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDynamicOverflowItemsChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDynamicOverflowItemsChanging: usize,
}
#[repr(C)]
pub struct ICommandBarElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCompact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCompact: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInOverflow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DynamicOverflowOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDynamicOverflowOrder: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PrimaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrimaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SecondaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SecondaryCommands: usize,
}
#[repr(C)]
pub struct ICommandBarFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarOverflowPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICommandBarOverflowPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrimaryCommandsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryCommandsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CommandBarOverflowPresenterStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultLabelPositionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OverflowButtonVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsDynamicOverflowEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContainerContentChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ItemContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ItemContainer: usize,
    pub InRecycleQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ItemIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Phase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RegisterUpdateCallback: unsafe extern "system" fn(this: *mut *mut Self, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterUpdateCallback: usize,
    #[cfg(feature = "Foundation")]
    pub RegisterUpdateCallbackWithPhase: unsafe extern "system" fn(this: *mut *mut Self, callbackphase: u32, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterUpdateCallbackWithPhase: usize,
}
#[repr(C)]
pub struct IContentControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ContentTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ContentTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetContentTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetContentTransitions: usize,
}
#[repr(C)]
pub struct IContentControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentTemplateRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentControlOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnContentChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentTemplateChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontenttemplate: *mut ::core::ffi::c_void, newcontenttemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentTemplateSelectorChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontenttemplateselector: *mut ::core::ffi::c_void, newcontenttemplateselector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialog {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TitleTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTitleTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FullSizeDesired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetFullSizeDesired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PrimaryButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSecondaryButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PrimaryButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PrimaryButtonCommand: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetPrimaryButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetPrimaryButtonCommand: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SecondaryButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SecondaryButtonCommand: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetSecondaryButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetSecondaryButtonCommand: usize,
    pub PrimaryButtonCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryButtonCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSecondaryButtonCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPrimaryButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPrimaryButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSecondaryButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSecondaryButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryButtonClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryButtonClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryButtonClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryButtonClick: usize,
    #[cfg(feature = "Foundation")]
    pub SecondaryButtonClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SecondaryButtonClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSecondaryButtonClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSecondaryButtonClick: usize,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
}
#[repr(C)]
pub struct IContentDialog2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CloseButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCloseButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub CloseButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    CloseButtonCommand: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetCloseButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetCloseButtonCommand: usize,
    pub CloseButtonCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCloseButtonCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSecondaryButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CloseButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCloseButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultButton: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContentDialogButton) -> ::windows_sys::core::HRESULT,
    pub SetDefaultButton: unsafe extern "system" fn(this: *mut *mut Self, value: ContentDialogButton) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CloseButtonClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseButtonClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCloseButtonClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCloseButtonClick: usize,
}
#[repr(C)]
pub struct IContentDialog3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, placement: ContentDialogPlacement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithPlacement: usize,
}
#[repr(C)]
pub struct IContentDialogButtonClickDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogButtonClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContentDialogResult) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogClosingDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContentDialogResult) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogOpenedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContentDialogStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TitleTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FullSizeDesiredProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryButtonTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryButtonCommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonCommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryButtonCommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonCommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPrimaryButtonEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSecondaryButtonEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentDialogStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CloseButtonTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CloseButtonCommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CloseButtonCommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryButtonStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryButtonStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CloseButtonStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultButtonProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentLinkChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContentLinkChangeKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub ContentLinkInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    ContentLinkInfo: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub TextRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Documents::TextRange) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    TextRange: usize,
}
#[repr(C)]
pub struct IContentPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ContentTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ContentTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetContentTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetContentTransitions: usize,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    pub CharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
}
#[repr(C)]
pub struct IContentPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpticalMarginAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::OpticalMarginAlignment) -> ::windows_sys::core::HRESULT,
    pub SetOpticalMarginAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::OpticalMarginAlignment) -> ::windows_sys::core::HRESULT,
    pub TextLineBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextLineBounds) -> ::windows_sys::core::HRESULT,
    pub SetTextLineBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextLineBounds) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenter3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenter4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextWrapping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLines: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    pub HorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenter5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundSizing) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnContentTemplateChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontenttemplate: *mut ::core::ffi::c_void, newcontenttemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentTemplateSelectorChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontenttemplateselector: *mut ::core::ffi::c_void, newcontenttemplateselector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpticalMarginAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextLineBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextWrappingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxLinesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentPresenterStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContextMenuEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CursorLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CursorTop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    pub CharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub IsTabStop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TabNavigation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::KeyboardNavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TabNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetTabNavigation: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::KeyboardNavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetTabNavigation: usize,
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub HorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBorderBrush: usize,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::FocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsEnabledChanged: usize,
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControl3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseSystemFocusVisuals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseSystemFocusVisuals: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControl4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFocusEngagementEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFocusEngagementEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFocusEngaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFocusEngaged: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RequiresPointer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RequiresPointer) -> ::windows_sys::core::HRESULT,
    pub SetRequiresPointer: unsafe extern "system" fn(this: *mut *mut Self, value: RequiresPointer) -> ::windows_sys::core::HRESULT,
    pub XYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FocusEngaged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusEngaged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusEngaged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusEngaged: usize,
    #[cfg(feature = "Foundation")]
    pub FocusDisengaged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusDisengaged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusDisengaged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusDisengaged: usize,
    pub RemoveFocusEngagement: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControl5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DefaultStyleResourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultStyleResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetDefaultStyleResourceUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultStyleResourceUri: usize,
}
#[repr(C)]
pub struct IControl7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerEntered: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerPressed: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerPressed: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerMoved: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerMoved: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerReleased: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerReleased: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerExited: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerExited: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerCaptureLost: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerCanceled: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerCanceled: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPointerWheelChanged: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnTapped: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnTapped: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnDoubleTapped: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnDoubleTapped: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnHolding: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnHolding: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnRightTapped: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnRightTapped: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnManipulationStarting: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnManipulationStarting: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnManipulationInertiaStarting: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnManipulationInertiaStarting: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnManipulationStarted: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnManipulationDelta: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnManipulationDelta: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnManipulationCompleted: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnKeyUp: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnKeyUp: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnKeyDown: usize,
    pub OnGotFocus: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnLostFocus: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDragEnter: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDragLeave: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDragOver: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDrop: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlOverrides6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPreviewKeyDown: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPreviewKeyDown: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnPreviewKeyUp: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnPreviewKeyUp: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnCharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnCharacterReceived: usize,
}
#[repr(C)]
pub struct IControlProtected {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultStyleKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultStyleKey: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTemplateChild: unsafe extern "system" fn(this: *mut *mut Self, childname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTabStopProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TabNavigationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultStyleKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseSystemFocusVisualsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTemplateFocusTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsTemplateFocusTarget: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTemplateFocusTarget: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFocusEngagementEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFocusEngagedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequiresPointerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusLeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultStyleResourceUriProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTemplateKeyTipTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsTemplateKeyTipTarget: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTemplateKeyTipTarget: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlTemplate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub TargetType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    TargetType: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SetTargetType: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SetTargetType: usize,
}
#[repr(C)]
pub struct IDataTemplateSelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectTemplate: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateSelector2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectTemplateForItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateSelectorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateSelectorOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectTemplateCore: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateSelectorOverrides2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectTemplateForItemCore: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OldDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldDate: usize,
    #[cfg(feature = "Foundation")]
    pub NewDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDate: usize,
}
#[repr(C)]
pub struct IDatePicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    pub DayVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDayVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MonthVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMonthVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub YearVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetYearVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DayFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDayFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MonthFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMonthFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YearFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetYearFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MinYear: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinYear: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinYear: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinYear: usize,
    #[cfg(feature = "Foundation")]
    pub MaxYear: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxYear: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxYear: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxYear: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDateChanged: usize,
}
#[repr(C)]
pub struct IDatePicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SelectedDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectedDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SelectedDateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedDateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectedDateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectedDateChanged: usize,
}
#[repr(C)]
pub struct IDatePickerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    pub DayVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDayVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MonthVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMonthVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub YearVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetYearVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MinYear: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinYear: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinYear: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinYear: usize,
    #[cfg(feature = "Foundation")]
    pub MaxYear: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxYear: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxYear: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxYear: usize,
    #[cfg(feature = "Foundation")]
    pub DatePicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DatePicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDatePicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDatePicked: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAtAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAtAsync: usize,
}
#[repr(C)]
pub struct IDatePickerFlyout2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DayFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDayFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MonthFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMonthFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YearFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetYearFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyoutItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrimaryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecondaryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSecondaryText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyoutItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrimaryTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyoutPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDatePickerFlyoutPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyoutPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CalendarIdentifierProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub YearVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinYearProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxYearProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerFlyoutStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DayFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub YearFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerSelectedValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OldDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldDate: usize,
    #[cfg(feature = "Foundation")]
    pub NewDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDate: usize,
}
#[repr(C)]
pub struct IDatePickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CalendarIdentifierProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub YearVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DayFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MonthFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub YearFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinYearProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxYearProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedDateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OldDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldDate: usize,
    #[cfg(feature = "Foundation")]
    pub NewDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDate: usize,
}
#[repr(C)]
pub struct IDragItemsCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub DropResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    DropResult: usize,
}
#[repr(C)]
pub struct IDragItemsStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Data: usize,
}
#[repr(C)]
pub struct IDropDownButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDropDownButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDropDownButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropDownButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDynamicOverflowItemsChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CommandBarDynamicOverflowAction) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlipView {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlipView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseTouchAnimationsForAllNavigation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseTouchAnimationsForAllNavigation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlipViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlipViewItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlipViewItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlipViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseTouchAnimationsForAllNavigationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlyoutPresenterStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFlyoutPresenterStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlyoutPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlyoutPresenterStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusDisengagedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFocusEngagedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFocusEngagedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIcon {
    pub base__: ::windows_sys::core::IInspectable,
    pub Glyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
}
#[repr(C)]
pub struct IFontIcon2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIcon3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MirroredWhenRightToLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMirroredWhenRightToLeft: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Glyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MirroredWhenRightToLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMirroredWhenRightToLeft: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MirroredWhenRightToLeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontIconStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MirroredWhenRightToLeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub CacheSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCacheSize: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub CurrentSourcePageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    CurrentSourcePageType: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SourcePageType: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SetSourcePageType: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SetSourcePageType: usize,
    pub BackStackDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Navigation"))]
    pub Navigated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Navigation")))]
    Navigated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigated: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Navigation"))]
    pub Navigating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Navigation")))]
    Navigating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigating: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigating: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Navigation"))]
    pub NavigationFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Navigation")))]
    NavigationFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationFailed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Navigation"))]
    pub NavigationStopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Navigation")))]
    NavigationStopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStopped: usize,
    pub GoBack: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GoForward: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Navigate: unsafe extern "system" fn(this: *mut *mut Self, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Navigate: usize,
    pub GetNavigationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNavigationState: unsafe extern "system" fn(this: *mut *mut Self, navigationstate: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Navigation"))]
    pub BackStack: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Navigation")))]
    BackStack: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Navigation"))]
    pub ForwardStack: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Navigation")))]
    ForwardStack: usize,
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))]
    pub Navigate: unsafe extern "system" fn(this: *mut *mut Self, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, infooverride: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation")))]
    Navigate: usize,
}
#[repr(C)]
pub struct IFrame3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub GoBack: unsafe extern "system" fn(this: *mut *mut Self, transitioninfooverride: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    GoBack: usize,
}
#[repr(C)]
pub struct IFrame4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetNavigationStateWithNavigationControl: unsafe extern "system" fn(this: *mut *mut Self, navigationstate: ::windows_sys::core::HSTRING, suppressnavigate: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrame5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsNavigationStackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsNavigationStackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Navigation"))]
    pub NavigateToType: unsafe extern "system" fn(this: *mut *mut Self, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, navigationoptions: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Navigation")))]
    NavigateToType: usize,
}
#[repr(C)]
pub struct IFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CacheSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanGoBackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanGoForwardProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CurrentSourcePageTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourcePageTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackStackDepthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackStackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForwardStackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsNavigationStackEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGrid {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RowDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RowDefinitions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ColumnDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ColumnDefinitions: usize,
}
#[repr(C)]
pub struct IGrid2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGrid3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RowSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRowSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ColumnSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetColumnSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGrid4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundSizing) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRow: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRow: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub ColumnProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetColumn: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub RowSpanProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRowSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRowSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub ColumnSpanProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetColumnSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetColumnSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RowSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColumnSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridView {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewHeaderItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridViewHeaderItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IGridViewItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGroupItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupStyle {
    pub base__: ::windows_sys::core::IInspectable,
    pub Panel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPanel: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContainerStyle: usize,
    #[cfg(feature = "deprecated")]
    pub SetContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetContainerStyle: usize,
    #[cfg(feature = "deprecated")]
    pub ContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContainerStyleSelector: usize,
    #[cfg(feature = "deprecated")]
    pub SetContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetContainerStyleSelector: usize,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HidesIfEmpty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHidesIfEmpty: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupStyle2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupStyleFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupStyleSelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectGroupStyle: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void, level: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupStyleSelectorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupStyleSelectorOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectGroupStyleCore: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void, level: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingPanelClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHandwritingPanelOpenedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHandwritingView {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlacementAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HandwritingPanelPlacementAlignment) -> ::windows_sys::core::HRESULT,
    pub SetPlacementAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: HandwritingPanelPlacementAlignment) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AreCandidatesEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreCandidatesEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub TryClose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSwitchToKeyboardEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSwitchToKeyboardEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCommandBarOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCommandBarOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub InputDeviceTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    InputDeviceTypes: usize,
    #[cfg(feature = "UI_Core")]
    pub SetInputDeviceTypes: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetInputDeviceTypes: usize,
    #[cfg(feature = "Foundation")]
    pub CandidatesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CandidatesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCandidatesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCandidatesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TextSubmitted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextSubmitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextSubmitted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextSubmitted: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidates: unsafe extern "system" fn(this: *mut *mut Self, candidatessessionid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidates: usize,
    pub SelectCandidate: unsafe extern "system" fn(this: *mut *mut Self, candidatessessionid: u32, selectedcandidateindex: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingViewCandidatesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CandidatesSessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlacementAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AreCandidatesEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSwitchToKeyboardEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCommandBarOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHandwritingViewTextSubmittedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHub {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    pub DefaultSectionIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultSectionIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Sections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SectionsInView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SectionsInView: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SectionHeaders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SectionHeaders: usize,
    #[cfg(feature = "Foundation")]
    pub SectionHeaderClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SectionHeaderClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSectionHeaderClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSectionHeaderClick: usize,
    #[cfg(feature = "Foundation")]
    pub SectionsInViewChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SectionsInViewChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSectionsInViewChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSectionsInViewChanged: usize,
    pub ScrollToSection: unsafe extern "system" fn(this: *mut *mut Self, section: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHubFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHubSection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHeaderInteractive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHeaderInteractive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHubSectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHubSectionHeaderClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Section: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHubSectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHeaderInteractiveProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHubStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultSectionIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SemanticZoomOwnerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsActiveViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomedInViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkButton {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NavigateUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigateUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetNavigateUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNavigateUri: usize,
}
#[repr(C)]
pub struct IHyperlinkButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub NavigateUriProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIconElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
}
#[repr(C)]
pub struct IIconElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IIconElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIconSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
}
#[repr(C)]
pub struct IIconSourceElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIconSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIconSourceElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIconSourceElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIconSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IIconSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Source: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    pub NineGrid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetNineGrid: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Media_PlayTo", feature = "deprecated"))]
    pub PlayToSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_PlayTo", feature = "deprecated")))]
    PlayToSource: usize,
    #[cfg(feature = "Foundation")]
    pub ImageFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageFailed: usize,
    #[cfg(feature = "Foundation")]
    pub ImageOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageOpened: usize,
}
#[repr(C)]
pub struct IImage2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Casting")]
    pub GetAsCastingSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Casting"))]
    GetAsCastingSource: usize,
}
#[repr(C)]
pub struct IImage3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub GetAlphaMask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetAlphaMask: usize,
}
#[repr(C)]
pub struct IImageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NineGridProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlayToSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayToSourceProperty: usize,
}
#[repr(C)]
pub struct IInkCanvas {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Input_Inking")]
    pub InkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Inking"))]
    InkPresenter: usize,
}
#[repr(C)]
pub struct IInkCanvasFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbar {
    pub base__: ::windows_sys::core::IInspectable,
    pub InitialControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarInitialControls) -> ::windows_sys::core::HRESULT,
    pub SetInitialControls: unsafe extern "system" fn(this: *mut *mut Self, value: InkToolbarInitialControls) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ActiveTool: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetActiveTool: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Input_Inking")]
    pub InkDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Inking"))]
    InkDrawingAttributes: usize,
    pub IsRulerButtonChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRulerButtonChecked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TargetInkCanvas: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetInkCanvas: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActiveToolChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActiveToolChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActiveToolChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActiveToolChanged: usize,
    #[cfg(feature = "Foundation")]
    pub InkDrawingAttributesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InkDrawingAttributesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInkDrawingAttributesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInkDrawingAttributesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub EraseAllClicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EraseAllClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEraseAllClicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEraseAllClicked: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IsRulerButtonCheckedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IsRulerButtonCheckedChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveIsRulerButtonCheckedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveIsRulerButtonCheckedChanged: usize,
    pub GetToolButton: unsafe extern "system" fn(this: *mut *mut Self, tool: InkToolbarTool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetToggleButton: unsafe extern "system" fn(this: *mut *mut Self, tool: InkToolbarToggle, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbar2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStencilButtonChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStencilButtonChecked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ButtonFlyoutPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarButtonFlyoutPlacement) -> ::windows_sys::core::HRESULT,
    pub SetButtonFlyoutPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: InkToolbarButtonFlyoutPlacement) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsStencilButtonCheckedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsStencilButtonCheckedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsStencilButtonCheckedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsStencilButtonCheckedChanged: usize,
    pub GetMenuButton: unsafe extern "system" fn(this: *mut *mut Self, menu: InkToolbarMenuKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbar3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Input_Inking")]
    pub TargetInkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Inking"))]
    TargetInkPresenter: usize,
    #[cfg(feature = "UI_Input_Inking")]
    pub SetTargetInkPresenter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Inking"))]
    SetTargetInkPresenter: usize,
}
#[repr(C)]
pub struct IInkToolbarBallpointPenButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarBallpointPenButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomPen {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
    pub CreateInkDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut ::core::ffi::c_void, strokewidth: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media")))]
    CreateInkDrawingAttributes: usize,
}
#[repr(C)]
pub struct IInkToolbarCustomPenButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub CustomPen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCustomPen: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConfigurationContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConfigurationContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomPenButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomPenButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CustomPenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConfigurationContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomPenFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomPenOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
    pub CreateInkDrawingAttributesCore: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut ::core::ffi::c_void, strokewidth: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media")))]
    CreateInkDrawingAttributesCore: usize,
}
#[repr(C)]
pub struct IInkToolbarCustomToggleButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarCustomToggleButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomToolButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConfigurationContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConfigurationContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomToolButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarCustomToolButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConfigurationContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarEraserButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarEraserButton2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsClearAllVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsClearAllVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarEraserButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarEraserButtonStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsClearAllVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarFlyoutItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarFlyoutItemKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: InkToolbarFlyoutItemKind) -> ::windows_sys::core::HRESULT,
    pub IsChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Checked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Checked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChecked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChecked: usize,
    #[cfg(feature = "Foundation")]
    pub Unchecked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Unchecked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnchecked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnchecked: usize,
}
#[repr(C)]
pub struct IInkToolbarFlyoutItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarFlyoutItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KindProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCheckedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarHighlighterButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarHighlighterButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarIsStencilButtonCheckedChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub StencilButton: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StencilKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarStencilKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarMenuButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub MenuKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarMenuKind) -> ::windows_sys::core::HRESULT,
    pub IsExtensionGlyphShown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExtensionGlyphShown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarMenuButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarMenuButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsExtensionGlyphShownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarPenButton {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Palette: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Palette: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetPalette: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetPalette: usize,
    pub MinStrokeWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinStrokeWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MaxStrokeWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxStrokeWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBrush: usize,
    pub SelectedBrushIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectedBrushIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectedStrokeWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSelectedStrokeWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarPenButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarPenButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaletteProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinStrokeWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxStrokeWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBrushIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedStrokeWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarPenConfigurationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub PenButton: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarPenConfigurationControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarPenConfigurationControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PenButtonProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarPencilButton {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarPencilButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IInkToolbarRulerButton {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "UI_Input_Inking", feature = "deprecated"))]
    pub Ruler: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Input_Inking", feature = "deprecated")))]
    Ruler: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IInkToolbarRulerButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IInkToolbarRulerButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub RulerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RulerProperty: usize,
}
#[repr(C)]
pub struct IInkToolbarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub InitialControlsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActiveToolProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InkDrawingAttributesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRulerButtonCheckedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetInkCanvasProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStencilButtonCheckedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ButtonFlyoutPlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetInkPresenterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarStencilButton {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Input_Inking")]
    pub Ruler: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Inking"))]
    Ruler: usize,
    #[cfg(feature = "UI_Input_Inking")]
    pub Protractor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Inking"))]
    Protractor: usize,
    pub SelectedStencil: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarStencilKind) -> ::windows_sys::core::HRESULT,
    pub SetSelectedStencil: unsafe extern "system" fn(this: *mut *mut Self, value: InkToolbarStencilKind) -> ::windows_sys::core::HRESULT,
    pub IsRulerItemVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRulerItemVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsProtractorItemVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsProtractorItemVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarStencilButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarStencilButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RulerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProtractorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedStencilProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRulerItemVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsProtractorItemVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarToggleButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToggleKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarToggle) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarToggleButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarToolButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToolKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkToolbarTool) -> ::windows_sys::core::HRESULT,
    pub IsExtensionGlyphShown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExtensionGlyphShown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkToolbarToolButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInkToolbarToolButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsExtensionGlyphShownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInsertionPanel {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetInsertionIndexes: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::super::Foundation::Point, first: *mut i32, second: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInsertionIndexes: usize,
}
#[repr(C)]
pub struct IIsTextTrimmedChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IItemClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClickedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemContainerGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub ItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))]
    ItemsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub ItemFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ItemFromContainer: usize,
    #[cfg(feature = "deprecated")]
    pub ContainerFromItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContainerFromItem: usize,
    #[cfg(feature = "deprecated")]
    pub IndexFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IndexFromContainer: usize,
    #[cfg(feature = "deprecated")]
    pub ContainerFromIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContainerFromIndex: usize,
    pub GetItemContainerGeneratorForPanel: unsafe extern "system" fn(this: *mut *mut Self, panel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub StartAt: unsafe extern "system" fn(this: *mut *mut Self, position: Primitives::GeneratorPosition, direction: Primitives::GeneratorDirection, allowstartatrealizeditem: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    StartAt: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GenerateNext: unsafe extern "system" fn(this: *mut *mut Self, isnewlyrealized: *mut bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrepareItemContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, position: Primitives::GeneratorPosition, count: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Remove: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub GeneratorPositionFromIndex: unsafe extern "system" fn(this: *mut *mut Self, itemindex: i32, result__: *mut Primitives::GeneratorPosition) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    GeneratorPositionFromIndex: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub IndexFromGeneratorPosition: unsafe extern "system" fn(this: *mut *mut Self, position: Primitives::GeneratorPosition, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    IndexFromGeneratorPosition: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Recycle: unsafe extern "system" fn(this: *mut *mut Self, position: Primitives::GeneratorPosition, count: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Recycle: usize,
}
#[repr(C)]
pub struct IItemContainerMapping {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContainerFromItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IndexFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ContainerFromIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsPanel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsPanel: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayMemberPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMemberPath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerGenerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ItemContainerTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ItemContainerTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetItemContainerTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetItemContainerTransitions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GroupStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GroupStyle: usize,
    pub GroupStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGroupStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsGrouping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsPanelRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControl3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupHeaderContainerFromItemContainer: unsafe extern "system" fn(this: *mut *mut Self, itemcontainer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControlOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsItemItsOwnContainerOverride: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetContainerForItemOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearContainerForItemOverride: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrepareContainerForItemOverride: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnItemContainerStyleChanged: unsafe extern "system" fn(this: *mut *mut Self, olditemcontainerstyle: *mut ::core::ffi::c_void, newitemcontainerstyle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnItemContainerStyleSelectorChanged: unsafe extern "system" fn(this: *mut *mut Self, olditemcontainerstyleselector: *mut ::core::ffi::c_void, newitemcontainerstyleselector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnItemTemplateChanged: unsafe extern "system" fn(this: *mut *mut Self, olditemtemplate: *mut ::core::ffi::c_void, newitemtemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnItemTemplateSelectorChanged: unsafe extern "system" fn(this: *mut *mut Self, olditemtemplateselector: *mut ::core::ffi::c_void, newitemtemplateselector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnGroupStyleSelectorChanged: unsafe extern "system" fn(this: *mut *mut Self, oldgroupstyleselector: *mut ::core::ffi::c_void, newgroupstyleselector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayMemberPathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyleSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GroupStyleSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsGroupingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetItemsOwner: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsControlFromItemContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsPanelTemplate {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IItemsPickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedItems: usize,
}
#[repr(C)]
pub struct IItemsPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub HeaderTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    HeaderTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetHeaderTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetHeaderTransitions: usize,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Footer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFooter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFooterTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub FooterTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    FooterTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetFooterTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetFooterTransitions: usize,
}
#[repr(C)]
pub struct IItemsPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FooterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsStackPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetGroupPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    pub FirstCacheIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FirstVisibleIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastVisibleIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastCacheIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ScrollingDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PanelScrollingDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub GroupHeaderPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::GroupHeaderPlacement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    GroupHeaderPlacement: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetGroupHeaderPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::GroupHeaderPlacement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetGroupHeaderPlacement: usize,
    pub ItemsUpdatingScrollMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ItemsUpdatingScrollMode) -> ::windows_sys::core::HRESULT,
    pub SetItemsUpdatingScrollMode: unsafe extern "system" fn(this: *mut *mut Self, value: ItemsUpdatingScrollMode) -> ::windows_sys::core::HRESULT,
    pub CacheLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCacheLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsStackPanel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreStickyGroupHeadersEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreStickyGroupHeadersEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsStackPanelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupPaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GroupHeaderPlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CacheLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsStackPanelStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreStickyGroupHeadersEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsWrapGrid {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetGroupPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    pub MaximumRowsOrColumns: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaximumRowsOrColumns: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ItemWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetItemWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ItemHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetItemHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FirstCacheIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FirstVisibleIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastVisibleIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastCacheIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ScrollingDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PanelScrollingDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub GroupHeaderPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::GroupHeaderPlacement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    GroupHeaderPlacement: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetGroupHeaderPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::GroupHeaderPlacement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetGroupHeaderPlacement: usize,
    pub CacheLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCacheLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsWrapGrid2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreStickyGroupHeadersEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreStickyGroupHeadersEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsWrapGridStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupPaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaximumRowsOrColumnsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GroupHeaderPlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CacheLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsWrapGridStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreStickyGroupHeadersEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBox {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectedItems: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: SelectionMode) -> ::windows_sys::core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SingleSelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSingleSelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBoxItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListBoxItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBoxStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SingleSelectionFollowsFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListPickerFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayMemberPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMemberPath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ListPickerFlyoutSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: ListPickerFlyoutSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValuePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSelectedValuePath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectedItems: usize,
    #[cfg(feature = "Foundation")]
    pub ItemsPicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemsPicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemsPicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemsPicked: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowAtAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowAtAsync: usize,
}
#[repr(C)]
pub struct IListPickerFlyoutPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListPickerFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayMemberPathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValuePathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListView {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewBase {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectedItems: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ListViewSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: ListViewSelectionMode) -> ::windows_sys::core::HRESULT,
    pub IsSwipeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSwipeEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanDragItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanDragItems: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanReorderItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanReorderItems: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsItemClickEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsItemClickEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DataFetchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDataFetchSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IncrementalLoadingThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetIncrementalLoadingThreshold: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IncrementalLoadingTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IncrementalLoadingTrigger) -> ::windows_sys::core::HRESULT,
    pub SetIncrementalLoadingTrigger: unsafe extern "system" fn(this: *mut *mut Self, value: IncrementalLoadingTrigger) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemClick: usize,
    #[cfg(feature = "Foundation")]
    pub DragItemsStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragItemsStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragItemsStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragItemsStarting: usize,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Data"))]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Data")))]
    LoadMoreItemsAsync: usize,
    pub ScrollIntoViewWithAlignment: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, alignment: ScrollIntoViewAlignment) -> ::windows_sys::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub HeaderTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    HeaderTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetHeaderTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetHeaderTransitions: usize,
}
#[repr(C)]
pub struct IListViewBase2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowsScrollingPlaceholders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowsScrollingPlaceholders: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContainerContentChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainerContentChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContainerContentChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContainerContentChanging: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredContainerUpdateDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredContainerUpdateDuration: usize,
    pub Footer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFooter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFooterTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub FooterTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    FooterTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetFooterTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetFooterTransitions: usize,
}
#[repr(C)]
pub struct IListViewBase3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReorderMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ListViewReorderMode) -> ::windows_sys::core::HRESULT,
    pub SetReorderMode: unsafe extern "system" fn(this: *mut *mut Self, value: ListViewReorderMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBase4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Data"))]
    pub SelectedRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Data")))]
    SelectedRanges: usize,
    pub IsMultiSelectCheckBoxEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMultiSelectCheckBoxEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DragItemsCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragItemsCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragItemsCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragItemsCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ChoosingItemContainer: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChoosingItemContainer: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChoosingItemContainer: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChoosingItemContainer: usize,
    #[cfg(feature = "Foundation")]
    pub ChoosingGroupHeaderContainer: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChoosingGroupHeaderContainer: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChoosingGroupHeaderContainer: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChoosingGroupHeaderContainer: usize,
    #[cfg(feature = "UI_Xaml_Data")]
    pub SelectRange: unsafe extern "system" fn(this: *mut *mut Self, itemindexrange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Data"))]
    SelectRange: usize,
    #[cfg(feature = "UI_Xaml_Data")]
    pub DeselectRange: unsafe extern "system" fn(this: *mut *mut Self, itemindexrange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Data"))]
    DeselectRange: usize,
}
#[repr(C)]
pub struct IListViewBase5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SingleSelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSingleSelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDragSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBase6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media_Animation"))]
    pub TryStartConnectedAnimationAsync: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, elementname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media_Animation")))]
    TryStartConnectedAnimationAsync: usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub PrepareConnectedAnimation: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, item: *mut ::core::ffi::c_void, elementname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    PrepareConnectedAnimation: usize,
}
#[repr(C)]
pub struct IListViewBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBaseHeaderItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewBaseHeaderItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSwipeEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanDragItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanReorderItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsItemClickEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DataFetchSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IncrementalLoadingThresholdProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IncrementalLoadingTriggerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SemanticZoomOwnerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsActiveViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomedInViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBaseStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowsScrollingPlaceholdersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FooterTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBaseStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReorderModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBaseStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMultiSelectCheckBoxEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewBaseStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SingleSelectionFollowsFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewHeaderItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewHeaderItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IListViewItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewPersistenceHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewPersistenceHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetRelativeScrollPosition: unsafe extern "system" fn(this: *mut *mut Self, listviewbase: *mut ::core::ffi::c_void, itemtokeyhandler: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRelativeScrollPositionAsync: unsafe extern "system" fn(this: *mut *mut Self, listviewbase: *mut ::core::ffi::c_void, relativescrollposition: ::windows_sys::core::HSTRING, keytoitemhandler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRelativeScrollPositionAsync: usize,
}
#[repr(C)]
pub struct IMediaElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PosterSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PosterSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPosterSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPosterSource: usize,
    #[cfg(feature = "Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Source: usize,
    #[cfg(feature = "Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSource: usize,
    pub IsMuted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAudioOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Balance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetBalance: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub NaturalVideoHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NaturalVideoWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NaturalDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub BufferingProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DownloadProgressOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CurrentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::MediaElementState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CurrentState: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Markers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Markers: usize,
    pub CanSeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanPause: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AudioStreamCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioStreamIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioStreamIndex: usize,
    #[cfg(feature = "Foundation")]
    pub SetAudioStreamIndex: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAudioStreamIndex: usize,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsLooping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLooping: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Media_PlayTo", feature = "deprecated"))]
    pub PlayToSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_PlayTo", feature = "deprecated")))]
    PlayToSource: usize,
    pub DefaultPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDefaultPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub AspectRatioWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AspectRatioHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RealTimePlayback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRealTimePlayback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub AudioCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::AudioCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    AudioCategory: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetAudioCategory: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::AudioCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetAudioCategory: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub AudioDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::AudioDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    AudioDeviceType: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetAudioDeviceType: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::AudioDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetAudioDeviceType: usize,
    #[cfg(feature = "Media_Protection")]
    pub ProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    ProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub SetProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetProtectionManager: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stereo3DVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stereo3DVideoPackingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stereo3DVideoPackingMode: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStereo3DVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stereo3DVideoPackingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStereo3DVideoPackingMode: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stereo3DVideoRenderMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stereo3DVideoRenderMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stereo3DVideoRenderMode: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStereo3DVideoRenderMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stereo3DVideoRenderMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStereo3DVideoRenderMode: usize,
    pub IsStereo3DVideo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MediaOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaOpened: usize,
    #[cfg(feature = "Foundation")]
    pub MediaEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaEnded: usize,
    #[cfg(feature = "Foundation")]
    pub MediaFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaFailed: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media"))]
    pub MarkerReached: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media")))]
    MarkerReached: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMarkerReached: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMarkerReached: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media"))]
    pub RateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media")))]
    RateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VolumeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VolumeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVolumeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVolumeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SeekCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeekCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSeekCompleted: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CanPlayType: unsafe extern "system" fn(this: *mut *mut Self, r#type: ::windows_sys::core::HSTRING, result__: *mut super::Media::MediaCanPlayResponse) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CanPlayType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSource2: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, mimetype: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSource2: usize,
    #[cfg(feature = "Foundation")]
    pub GetAudioStreamLanguage: unsafe extern "system" fn(this: *mut *mut Self, index: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioStreamLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut *mut Self, effectid: ::windows_sys::core::HSTRING, effectoptional: bool, effectconfiguration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut *mut Self, effectid: ::windows_sys::core::HSTRING, effectoptional: bool, effectconfiguration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffect: usize,
    pub RemoveAllEffects: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub ActualStereo3DVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stereo3DVideoPackingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    ActualStereo3DVideoPackingMode: usize,
}
#[repr(C)]
pub struct IMediaElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreTransportControlsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreTransportControlsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    pub IsFullWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFullWindow: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub SetMediaStreamSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetMediaStreamSource: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayToPreferredSourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayToPreferredSourceUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPlayToPreferredSourceUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPlayToPreferredSourceUri: usize,
}
#[repr(C)]
pub struct IMediaElement3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransportControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransportControls: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media"))]
    pub PartialMediaFailureDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media")))]
    PartialMediaFailureDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePartialMediaFailureDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePartialMediaFailureDetected: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetPlaybackSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetPlaybackSource: usize,
    #[cfg(feature = "Media_Casting")]
    pub GetAsCastingSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Casting"))]
    GetAsCastingSource: usize,
}
#[repr(C)]
pub struct IMediaElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PosterSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsMutedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAudioOnlyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VolumeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BalanceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalVideoHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalVideoWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalDurationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PositionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DownloadProgressProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BufferingProgressProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DownloadProgressOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CurrentStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanSeekProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanPauseProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AudioStreamCountProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AudioStreamIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaybackRateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsLoopingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlayToSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayToSourceProperty: usize,
    pub DefaultPlaybackRateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AspectRatioWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AspectRatioHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RealTimePlaybackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AudioCategoryProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AudioDeviceTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProtectionManagerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Stereo3DVideoPackingModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Stereo3DVideoRenderModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStereo3DVideoProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActualStereo3DVideoPackingModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaElementStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreTransportControlsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFullWindowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlayToPreferredSourceUriProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayToPreferredSourceUriProperty: usize,
}
#[repr(C)]
pub struct IMediaPlayerElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Playback")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    Source: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetSource: usize,
    pub TransportControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransportControls: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AreTransportControlsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreTransportControlsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PosterSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PosterSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPosterSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPosterSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    pub AutoPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFullWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFullWindow: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Playback")]
    pub MediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    MediaPlayer: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetMediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, mediaplayer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetMediaPlayer: usize,
}
#[repr(C)]
pub struct IMediaPlayerElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaPlayerElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AreTransportControlsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PosterSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFullWindowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MediaPlayerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaPlayerPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Playback")]
    pub MediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    MediaPlayer: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetMediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetMediaPlayer: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    pub IsFullWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFullWindow: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaPlayerPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaPlayerPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaPlayerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFullWindowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControls {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFullWindowButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFullWindowButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFullWindowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFullWindowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFastForwardButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFastForwardButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFastForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFastRewindButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFastRewindButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFastRewindEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFastRewindEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsStopButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStopButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStopEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVolumeButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVolumeButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVolumeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVolumeEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPlaybackRateButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPlaybackRateButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPlaybackRateEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSeekBarVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSeekBarVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSeekEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSeekEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCompact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCompact: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControls2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSkipForwardButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSkipForwardButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSkipForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSkipForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSkipBackwardButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSkipBackwardButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSkipBackwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSkipBackwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsNextTrackButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsNextTrackButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPreviousTrackButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPreviousTrackButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FastPlayFallbackBehaviour: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::FastPlayFallbackBehaviour) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FastPlayFallbackBehaviour: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFastPlayFallbackBehaviour: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::FastPlayFallbackBehaviour) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFastPlayFallbackBehaviour: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media"))]
    pub ThumbnailRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media")))]
    ThumbnailRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThumbnailRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThumbnailRequested: usize,
}
#[repr(C)]
pub struct IMediaTransportControls3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowAndHideAutomatically: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowAndHideAutomatically: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRepeatEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRepeatButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRepeatButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControls4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCompactOverlayButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCompactOverlayButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCompactOverlayEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCompactOverlayEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControlsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControlsHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMediaTransportControlsHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DropoutOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDropoutOrder: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDropoutOrder: usize,
    #[cfg(feature = "Foundation")]
    pub SetDropoutOrder: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDropoutOrder: usize,
}
#[repr(C)]
pub struct IMediaTransportControlsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFullWindowButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFullWindowEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFastForwardButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFastForwardEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFastRewindButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFastRewindEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStopButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStopEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsVolumeButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsVolumeEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPlaybackRateButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPlaybackRateEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSeekBarVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSeekEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCompactProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControlsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSkipForwardButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSkipForwardEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSkipBackwardButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSkipBackwardEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsNextTrackButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPreviousTrackButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FastPlayFallbackBehaviourProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControlsStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowAndHideAutomaticallyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRepeatEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRepeatButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControlsStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCompactOverlayButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCompactOverlayEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuBar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[repr(C)]
pub struct IMenuBarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuBarItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[repr(C)]
pub struct IMenuBarItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuBarItemFlyout {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuBarItemFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuBarItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub MenuFlyoutPresenterStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMenuFlyoutPresenterStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyout2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAt: unsafe extern "system" fn(this: *mut *mut Self, targetelement: *mut ::core::ffi::c_void, point: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAt: usize,
}
#[repr(C)]
pub struct IMenuFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
    pub Click: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Click: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClick: usize,
}
#[repr(C)]
pub struct IMenuFlyoutItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutItem3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetKeyboardAcceleratorTextOverride: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IMenuFlyoutItemBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuFlyoutItemBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuFlyoutItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutItemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutItemStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextOverrideProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuFlyoutPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IMenuFlyoutPresenter3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutPresenterStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutSeparator {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuFlyoutSeparatorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MenuFlyoutPresenterStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutSubItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutSubItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutSubItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutSubItemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Navigate: unsafe extern "system" fn(this: *mut *mut Self, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Navigate: usize,
}
#[repr(C)]
pub struct INavigationView {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPaneOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPaneOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CompactModeThresholdWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCompactModeThresholdWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ExpandedModeThresholdWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetExpandedModeThresholdWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub PaneFooter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPaneFooter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewDisplayMode) -> ::windows_sys::core::HRESULT,
    pub IsSettingsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSettingsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPaneToggleButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPaneToggleButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlwaysShowHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysShowHeader: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CompactPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCompactPaneLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OpenPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpenPaneLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub PaneToggleButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPaneToggleButtonStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MenuItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MenuItems: usize,
    pub MenuItemsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMenuItemsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SettingsItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoSuggestBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAutoSuggestBox: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMenuItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMenuItemTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMenuItemContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMenuItemContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContainerFromMenuItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub DisplayModeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayModeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayModeChanged: usize,
}
#[repr(C)]
pub struct INavigationView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBackButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewBackButtonVisible) -> ::windows_sys::core::HRESULT,
    pub SetIsBackButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: NavigationViewBackButtonVisible) -> ::windows_sys::core::HRESULT,
    pub IsBackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PaneTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPaneTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BackRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PaneClosed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneClosed: usize,
    #[cfg(feature = "Foundation")]
    pub PaneClosing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneClosing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneClosing: usize,
    #[cfg(feature = "Foundation")]
    pub PaneOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneOpened: usize,
    #[cfg(feature = "Foundation")]
    pub PaneOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneOpening: usize,
}
#[repr(C)]
pub struct INavigationView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaneDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewPaneDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetPaneDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, value: NavigationViewPaneDisplayMode) -> ::windows_sys::core::HRESULT,
    pub PaneHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPaneHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneCustomContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPaneCustomContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentOverlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentOverlay: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPaneVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPaneVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewSelectionFollowsFocus) -> ::windows_sys::core::HRESULT,
    pub SetSelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, value: NavigationViewSelectionFollowsFocus) -> ::windows_sys::core::HRESULT,
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShoulderNavigationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewShoulderNavigationEnabled) -> ::windows_sys::core::HRESULT,
    pub SetShoulderNavigationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: NavigationViewShoulderNavigationEnabled) -> ::windows_sys::core::HRESULT,
    pub OverflowLabelMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewOverflowLabelMode) -> ::windows_sys::core::HRESULT,
    pub SetOverflowLabelMode: unsafe extern "system" fn(this: *mut *mut Self, value: NavigationViewOverflowLabelMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewBackRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewDisplayModeChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationViewDisplayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompactPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectsOnInvoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSelectsOnInvoked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewItemBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemHeader {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewItemHeaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InvokedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSettingsInvoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemInvokedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InvokedItemContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub RecommendedNavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    RecommendedNavigationTransitionInfo: usize,
}
#[repr(C)]
pub struct INavigationViewItemSeparator {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewItemSeparatorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompactPaneLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectsOnInvokedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewList {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewListFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewPaneClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewSelectionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSettingsSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewSelectionChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedItemContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub RecommendedNavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    RecommendedNavigationTransitionInfo: usize,
}
#[repr(C)]
pub struct INavigationViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPaneOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompactModeThresholdWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExpandedModeThresholdWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneFooterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSettingsVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPaneToggleButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlwaysShowHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompactPaneLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenPaneLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneToggleButtonStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemsSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SettingsItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoSuggestBoxProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemTemplateSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemContainerStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MenuItemContainerStyleSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBackButtonVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsBackEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneTitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaneDisplayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneCustomContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentOverlayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPaneVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionFollowsFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TemplateSettingsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShoulderNavigationEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OverflowLabelModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub TopPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OverflowButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub PaneToggleButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub BackButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub TopPaneVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub LeftPaneVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub SingleSelectionFollowsFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewTemplateSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewTemplateSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TopPaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OverflowButtonVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneToggleButtonVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackButtonVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopPaneVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LeftPaneVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SingleSelectionFollowsFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INotifyEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INotifyEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CallingUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CallingUri: usize,
}
#[repr(C)]
pub struct IPage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Navigation")]
    pub NavigationCacheMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Navigation::NavigationCacheMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Navigation"))]
    NavigationCacheMode: usize,
    #[cfg(feature = "UI_Xaml_Navigation")]
    pub SetNavigationCacheMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::Navigation::NavigationCacheMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Navigation"))]
    SetNavigationCacheMode: usize,
    pub TopAppBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopAppBar: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BottomAppBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottomAppBar: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPageOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Navigation")]
    pub OnNavigatedFrom: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Navigation"))]
    OnNavigatedFrom: usize,
    #[cfg(feature = "UI_Xaml_Navigation")]
    pub OnNavigatedTo: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Navigation"))]
    OnNavigatedTo: usize,
    #[cfg(feature = "UI_Xaml_Navigation")]
    pub OnNavigatingFrom: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Navigation"))]
    OnNavigatingFrom: usize,
}
#[repr(C)]
pub struct IPageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopAppBarProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BottomAppBarProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPanel {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    pub IsItemsHost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ChildrenTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ChildrenTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetChildrenTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetChildrenTransitions: usize,
}
#[repr(C)]
pub struct IPanel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPanelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsItemsHostProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildrenTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IParallaxView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Child: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalShift: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalShift: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalSourceEndOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalSourceEndOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalSourceOffsetKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ParallaxSourceOffsetKind) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalSourceOffsetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ParallaxSourceOffsetKind) -> ::windows_sys::core::HRESULT,
    pub HorizontalSourceStartOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalSourceStartOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalShiftClamped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHorizontalShiftClamped: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVerticalShiftClamped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVerticalShiftClamped: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MaxHorizontalShiftRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxHorizontalShiftRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MaxVerticalShiftRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxVerticalShiftRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalShift: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalShift: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalSourceEndOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalSourceEndOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalSourceOffsetKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ParallaxSourceOffsetKind) -> ::windows_sys::core::HRESULT,
    pub SetVerticalSourceOffsetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ParallaxSourceOffsetKind) -> ::windows_sys::core::HRESULT,
    pub VerticalSourceStartOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalSourceStartOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RefreshAutomaticHorizontalOffsets: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RefreshAutomaticVerticalOffsets: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IParallaxViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IParallaxViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChildProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalSourceEndOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalSourceOffsetKindProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalSourceStartOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxHorizontalShiftRatioProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalShiftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalShiftClampedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsVerticalShiftClampedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalSourceEndOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalSourceOffsetKindProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalSourceStartOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxVerticalShiftRatioProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalShiftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub Password: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PasswordChar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPasswordChar: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsPasswordRevealButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPasswordRevealButtonEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPasswordRevealButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPasswordRevealButtonEnabled: usize,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PasswordChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasswordChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePasswordChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePasswordChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContextMenuOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextMenuOpening: usize,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColor: usize,
    pub PreventKeyboardDisplayOnProgrammaticFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreventKeyboardDisplayOnProgrammaticFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Paste: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paste: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaste: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaste: usize,
}
#[repr(C)]
pub struct IPasswordBox3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PasswordRevealMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PasswordRevealMode) -> ::windows_sys::core::HRESULT,
    pub SetPasswordRevealMode: unsafe extern "system" fn(this: *mut *mut Self, value: PasswordRevealMode) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub SetTextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub InputScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    InputScope: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetInputScope: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetInputScope: usize,
}
#[repr(C)]
pub struct IPasswordBox4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PasswordChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasswordChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePasswordChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePasswordChanging: usize,
}
#[repr(C)]
pub struct IPasswordBox5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanPasteClipboardContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetSelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetSelectionFlyout: usize,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PasteFromClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBoxPasswordChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentChanging: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PasswordProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PasswordCharProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsPasswordRevealButtonEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPasswordRevealButtonEnabledProperty: usize,
    pub MaxLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBoxStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionHighlightColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreventKeyboardDisplayOnProgrammaticFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBoxStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PasswordRevealModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InputScopeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordBoxStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanPasteClipboardContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathIcon {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Data: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetData: usize,
}
#[repr(C)]
pub struct IPathIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathIconSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Data: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetData: usize,
}
#[repr(C)]
pub struct IPathIconSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathIconSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathIconStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPersonPicture {
    pub base__: ::windows_sys::core::IInspectable,
    pub BadgeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBadgeNumber: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub BadgeGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBadgeGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BadgeImageSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BadgeImageSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBadgeImageSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBadgeImageSource: usize,
    pub BadgeText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBadgeText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGroup: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContact: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContact: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Initials: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetInitials: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PreferSmallImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreferSmallImage: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub ProfilePicture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    ProfilePicture: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetProfilePicture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetProfilePicture: usize,
}
#[repr(C)]
pub struct IPersonPictureFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPersonPictureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BadgeNumberProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BadgeGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BadgeImageSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BadgeTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsGroupProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContactProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InitialsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreferSmallImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProfilePictureProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPickerConfirmedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPickerFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConfirmationButtonsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetConfirmationButtonsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Confirmed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Confirmed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConfirmed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfirmed: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAtAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAtAsync: usize,
}
#[repr(C)]
pub struct IPickerFlyoutPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPickerFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConfirmationButtonsVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivot {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TitleTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTitleTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLocked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PivotItemLoading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PivotItemLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePivotItemLoading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePivotItemLoading: usize,
    #[cfg(feature = "Foundation")]
    pub PivotItemLoaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PivotItemLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePivotItemLoaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePivotItemLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub PivotItemUnloading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PivotItemUnloading: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePivotItemUnloading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePivotItemUnloading: usize,
    #[cfg(feature = "Foundation")]
    pub PivotItemUnloaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PivotItemUnloaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePivotItemUnloaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePivotItemUnloaded: usize,
}
#[repr(C)]
pub struct IPivot2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLeftHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LeftHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLeftHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRightHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRightHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivot3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderFocusVisualPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PivotHeaderFocusVisualPlacement) -> ::windows_sys::core::HRESULT,
    pub SetHeaderFocusVisualPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: PivotHeaderFocusVisualPlacement) -> ::windows_sys::core::HRESULT,
    pub IsHeaderItemsCarouselEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHeaderItemsCarouselEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotItemEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TitleTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsLockedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SlideInAnimationGroupProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSlideInAnimationGroup: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut PivotSlideInAnimationGroup) -> ::windows_sys::core::HRESULT,
    pub SetSlideInAnimationGroup: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: PivotSlideInAnimationGroup) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LeftHeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightHeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderFocusVisualPlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHeaderItemsCarouselEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProgressBar {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsIndeterminate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIndeterminate: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowError: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowPaused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowPaused: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IProgressBarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProgressBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsIndeterminateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowErrorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowPausedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProgressRing {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
}
#[repr(C)]
pub struct IProgressRingStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsActiveProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadioButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGroupName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadioButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadioButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InitialSetValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInitialSetValue: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub IsClearEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsClearEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MaxRating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxRating: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub PlaceholderValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ItemInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
}
#[repr(C)]
pub struct IRatingControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CaptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InitialSetValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsClearEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxRatingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemInfoProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingItemFontInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisabledGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisabledGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Glyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PointerOverGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPointerOverGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PointerOverPlaceholderGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPointerOverPlaceholderGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PlaceholderGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UnsetGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUnsetGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingItemFontInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingItemFontInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisabledGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverPlaceholderGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnsetGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingItemImageInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DisabledImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DisabledImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDisabledImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDisabledImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Image: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverPlaceholderImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverPlaceholderImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverPlaceholderImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverPlaceholderImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub UnsetImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    UnsetImage: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetUnsetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetUnsetImage: usize,
}
#[repr(C)]
pub struct IRatingItemImageInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingItemImageInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisabledImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverPlaceholderImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnsetImageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRatingItemInfo {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRatingItemInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Visualizer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVisualizer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PullDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RefreshPullDirection) -> ::windows_sys::core::HRESULT,
    pub SetPullDirection: unsafe extern "system" fn(this: *mut *mut Self, value: RefreshPullDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RefreshRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRefreshRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRefreshRequested: usize,
    pub RequestRefresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshContainerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshContainerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub VisualizerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PullDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshInteractionRatioChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IRefreshStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RefreshVisualizerState) -> ::windows_sys::core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RefreshVisualizerState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshVisualizer {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestRefresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RefreshVisualizerOrientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: RefreshVisualizerOrientation) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RefreshVisualizerState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RefreshRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRefreshRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRefreshRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RefreshStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRefreshStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRefreshStateChanged: usize,
}
#[repr(C)]
pub struct IRefreshVisualizerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRefreshVisualizerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub InfoProviderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativePanel {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativePanel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundSizing) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativePanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativePanelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftOfProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLeftOf: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLeftOf: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AboveProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAbove: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAbove: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightOfProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRightOf: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRightOf: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BelowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBelow: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBelow: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignHorizontalCenterWithProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignHorizontalCenterWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlignHorizontalCenterWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignVerticalCenterWithProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignVerticalCenterWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlignVerticalCenterWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignLeftWithProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignLeftWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlignLeftWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignTopWithProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignTopWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlignTopWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignRightWithProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignRightWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlignRightWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignBottomWithProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignBottomWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlignBottomWith: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignLeftWithPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignLeftWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignLeftWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlignTopWithPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignTopWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignTopWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlignRightWithPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignRightWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignRightWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlignBottomWithPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignBottomWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignBottomWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlignHorizontalCenterWithPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignHorizontalCenterWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignHorizontalCenterWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlignVerticalCenterWithPanelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAlignVerticalCenterWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignVerticalCenterWithPanel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub BorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativePanelStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AcceptsReturn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAcceptsReturn: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub TextWrapping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub IsSpellCheckEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSpellCheckEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsTextPredictionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextPredictionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub Document: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    Document: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub InputScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    InputScope: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetInputScope: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetInputScope: usize,
    #[cfg(feature = "Foundation")]
    pub TextChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContextMenuOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextMenuOpening: usize,
}
#[repr(C)]
pub struct IRichEditBox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColor: usize,
    pub PreventKeyboardDisplayOnProgrammaticFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreventKeyboardDisplayOnProgrammaticFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Paste: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paste: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaste: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaste: usize,
}
#[repr(C)]
pub struct IRichEditBox3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TextCompositionStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextCompositionStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub TextCompositionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextCompositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextCompositionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextCompositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TextCompositionEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextCompositionEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextCompositionEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextCompositionEnded: usize,
    pub TextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub SetTextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub DesiredCandidateWindowAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CandidateWindowAlignment) -> ::windows_sys::core::HRESULT,
    pub SetDesiredCandidateWindowAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: CandidateWindowAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CandidateWindowBoundsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CandidateWindowBoundsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCandidateWindowBoundsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCandidateWindowBoundsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TextChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextChanging: usize,
}
#[repr(C)]
pub struct IRichEditBox4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetLinguisticAlternativesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetLinguisticAlternativesAsync: usize,
    pub ClipboardCopyFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RichEditClipboardFormat) -> ::windows_sys::core::HRESULT,
    pub SetClipboardCopyFormat: unsafe extern "system" fn(this: *mut *mut Self, value: RichEditClipboardFormat) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBox5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColorWhenNotFocused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColorWhenNotFocused: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColorWhenNotFocused: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColorWhenNotFocused: usize,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBox6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub CharacterCasing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CharacterCasing) -> ::windows_sys::core::HRESULT,
    pub SetCharacterCasing: unsafe extern "system" fn(this: *mut *mut Self, value: CharacterCasing) -> ::windows_sys::core::HRESULT,
    pub DisabledFormattingAccelerators: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisabledFormattingAccelerators) -> ::windows_sys::core::HRESULT,
    pub SetDisabledFormattingAccelerators: unsafe extern "system" fn(this: *mut *mut Self, value: DisabledFormattingAccelerators) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CopyingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCopyingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCopyingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub CuttingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CuttingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCuttingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCuttingToClipboard: usize,
}
#[repr(C)]
pub struct IRichEditBox7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub ContentLinkForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    ContentLinkForegroundColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetContentLinkForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetContentLinkForegroundColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub ContentLinkBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    ContentLinkBackgroundColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetContentLinkBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetContentLinkBackgroundColor: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentLinkProviders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentLinkProviders: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub SetContentLinkProviders: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    SetContentLinkProviders: usize,
    pub HandwritingView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHandwritingView: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHandwritingViewEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHandwritingViewEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentLinkChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLinkChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentLinkChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentLinkChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Documents"))]
    pub ContentLinkInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Documents")))]
    ContentLinkInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentLinkInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentLinkInvoked: usize,
}
#[repr(C)]
pub struct IRichEditBox8 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Text")]
    pub TextDocument: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    TextDocument: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetSelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetSelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ProofingMenuFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ProofingMenuFlyout: usize,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanging: usize,
}
#[repr(C)]
pub struct IRichEditBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxSelectionChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SelectionLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AcceptsReturnProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextWrappingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSpellCheckEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextPredictionEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InputScopeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionHighlightColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreventKeyboardDisplayOnProgrammaticFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredCandidateWindowAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClipboardCopyFormatProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionHighlightColorWhenNotFocusedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterCasingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisabledFormattingAcceleratorsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentLinkForegroundColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentLinkBackgroundColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentLinkProvidersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HandwritingViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHandwritingViewEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxStatics8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProofingMenuFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditBoxTextChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRichEditBoxTextChangingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentChanging: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlock {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub TextWrapping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub TextTrimming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextTrimming) -> ::windows_sys::core::HRESULT,
    pub SetTextTrimming: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextTrimming) -> ::windows_sys::core::HRESULT,
    pub TextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents"))]
    pub Blocks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents")))]
    Blocks: usize,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub CharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub OverflowContentTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOverflowContentTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextSelectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextSelectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HasOverflowContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SelectedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentStart: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentEnd: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub SelectionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    SelectionStart: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub SelectionEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    SelectionEnd: usize,
    pub BaselineOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContextMenuOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextMenuOpening: usize,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub Select: unsafe extern "system" fn(this: *mut *mut Self, start: *mut ::core::ffi::c_void, end: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    Select: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Documents"))]
    pub GetPositionFromPoint: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Documents")))]
    GetPositionFromPoint: usize,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TextIndent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTextIndent: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlock2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLines: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TextLineBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextLineBounds) -> ::windows_sys::core::HRESULT,
    pub SetTextLineBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextLineBounds) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColor: usize,
    pub OpticalMarginAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::OpticalMarginAlignment) -> ::windows_sys::core::HRESULT,
    pub SetOpticalMarginAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::OpticalMarginAlignment) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub SetTextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlock3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlock4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Text")]
    pub TextDecorations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::TextDecorations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    TextDecorations: usize,
    #[cfg(feature = "UI_Text")]
    pub SetTextDecorations: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::TextDecorations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetTextDecorations: usize,
}
#[repr(C)]
pub struct IRichTextBlock5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextTrimmed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents"))]
    pub TextHighlighters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents")))]
    TextHighlighters: usize,
    #[cfg(feature = "Foundation")]
    pub IsTextTrimmedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsTextTrimmedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsTextTrimmedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsTextTrimmedChanged: usize,
}
#[repr(C)]
pub struct IRichTextBlock6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetSelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetSelectionFlyout: usize,
    pub CopySelectionToClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockOverflow {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverflowContentTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOverflowContentTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub ContentSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasOverflowContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentStart: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentEnd: usize,
    pub BaselineOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Documents"))]
    pub GetPositionFromPoint: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Documents")))]
    GetPositionFromPoint: usize,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockOverflow2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLines: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockOverflow3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextTrimmed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsTextTrimmedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsTextTrimmedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsTextTrimmedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsTextTrimmedChanged: usize,
}
#[repr(C)]
pub struct IRichTextBlockOverflowStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverflowContentTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasOverflowContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockOverflowStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxLinesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockOverflowStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextTrimmedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextWrappingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextTrimmingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OverflowContentTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextSelectionEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasOverflowContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextIndentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxLinesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextLineBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionHighlightColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpticalMarginAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextDecorationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextTrimmedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichTextBlockStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRowDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::GridLength) -> ::windows_sys::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::GridLength) -> ::windows_sys::core::HRESULT,
    pub MaxHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MinHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ActualHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRowDefinitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollAnchorProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentAnchor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterAnchorCandidate: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterAnchorCandidate: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollContentPresenter {
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
    pub MakeVisible: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, rectangle: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MakeVisible: usize,
}
#[repr(C)]
pub struct IScrollContentPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanContentRenderOutsideBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanContentRenderOutsideBounds: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SizesContentToTemplatedParent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSizesContentToTemplatedParent: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollContentPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanContentRenderOutsideBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SizesContentToTemplatedParentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewer {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, value: ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub VerticalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub SetVerticalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, value: ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHorizontalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVerticalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVerticalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHorizontalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVerticalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVerticalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsScrollInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsScrollInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ScrollMode) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, value: ScrollMode) -> ::windows_sys::core::HRESULT,
    pub VerticalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ScrollMode) -> ::windows_sys::core::HRESULT,
    pub SetVerticalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, value: ScrollMode) -> ::windows_sys::core::HRESULT,
    pub ZoomMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ZoomMode) -> ::windows_sys::core::HRESULT,
    pub SetZoomMode: unsafe extern "system" fn(this: *mut *mut Self, value: ZoomMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub HorizontalSnapPointsAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::SnapPointsAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    HorizontalSnapPointsAlignment: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetHorizontalSnapPointsAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::SnapPointsAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetHorizontalSnapPointsAlignment: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub VerticalSnapPointsAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::SnapPointsAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    VerticalSnapPointsAlignment: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetVerticalSnapPointsAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::SnapPointsAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetVerticalSnapPointsAlignment: usize,
    pub HorizontalSnapPointsType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SnapPointsType) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalSnapPointsType: unsafe extern "system" fn(this: *mut *mut Self, value: SnapPointsType) -> ::windows_sys::core::HRESULT,
    pub VerticalSnapPointsType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SnapPointsType) -> ::windows_sys::core::HRESULT,
    pub SetVerticalSnapPointsType: unsafe extern "system" fn(this: *mut *mut Self, value: SnapPointsType) -> ::windows_sys::core::HRESULT,
    pub ZoomSnapPointsType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SnapPointsType) -> ::windows_sys::core::HRESULT,
    pub SetZoomSnapPointsType: unsafe extern "system" fn(this: *mut *mut Self, value: SnapPointsType) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ScrollableWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ComputedHorizontalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ScrollableHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ComputedVerticalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MaxZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub ZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ZoomSnapPoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ZoomSnapPoints: usize,
    #[cfg(feature = "Foundation")]
    pub ViewChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewChanged: usize,
    #[cfg(feature = "deprecated")]
    pub ScrollToHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ScrollToHorizontalOffset: usize,
    #[cfg(feature = "deprecated")]
    pub ScrollToVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ScrollToVerticalOffset: usize,
    #[cfg(feature = "deprecated")]
    pub ZoomToFactor: unsafe extern "system" fn(this: *mut *mut Self, factor: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ZoomToFactor: usize,
    pub InvalidateScrollInfo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsDeferredScrollingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDeferredScrollingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BringIntoViewOnFocusChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBringIntoViewOnFocusChange: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TopLeftHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopLeftHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LeftHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLeftHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewChanging: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeView: unsafe extern "system" fn(this: *mut *mut Self, horizontaloffset: *mut ::core::ffi::c_void, verticaloffset: *mut ::core::ffi::c_void, zoomfactor: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeView: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeViewWithOptionalAnimation: unsafe extern "system" fn(this: *mut *mut Self, horizontaloffset: *mut ::core::ffi::c_void, verticaloffset: *mut ::core::ffi::c_void, zoomfactor: *mut ::core::ffi::c_void, disableanimation: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeViewWithOptionalAnimation: usize,
}
#[repr(C)]
pub struct IScrollViewer3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DirectManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DirectManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDirectManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDirectManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub DirectManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DirectManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDirectManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDirectManipulationCompleted: usize,
}
#[repr(C)]
pub struct IScrollViewer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReduceViewportForCoreInputViewOcclusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetReduceViewportForCoreInputViewOcclusions: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalAnchorRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalAnchorRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalAnchorRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalAnchorRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CanContentRenderOutsideBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanContentRenderOutsideBounds: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnchorRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnchorRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnchorRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnchorRequested: usize,
}
#[repr(C)]
pub struct IScrollViewerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalSnapPointsAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalSnapPointsAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalSnapPointsTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalSnapPointsTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomSnapPointsTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewportWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScrollableWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ComputedHorizontalScrollBarVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtentWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewportHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScrollableHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ComputedVerticalScrollBarVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtentHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinZoomFactorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxZoomFactorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomFactorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomSnapPointsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalScrollBarVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHorizontalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, horizontalscrollbarvisibility: ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub VerticalScrollBarVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVerticalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub SetVerticalScrollBarVisibility: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, verticalscrollbarvisibility: ScrollBarVisibility) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalRailEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsHorizontalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHorizontalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, ishorizontalrailenabled: bool) -> ::windows_sys::core::HRESULT,
    pub IsVerticalRailEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsVerticalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVerticalRailEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, isverticalrailenabled: bool) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalScrollChainingEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsHorizontalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHorizontalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, ishorizontalscrollchainingenabled: bool) -> ::windows_sys::core::HRESULT,
    pub IsVerticalScrollChainingEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsVerticalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVerticalScrollChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, isverticalscrollchainingenabled: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomChainingEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsZoomChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomChainingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, iszoomchainingenabled: bool) -> ::windows_sys::core::HRESULT,
    pub IsScrollInertiaEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsScrollInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsScrollInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, isscrollinertiaenabled: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomInertiaEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsZoomInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomInertiaEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, iszoominertiaenabled: bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalScrollModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHorizontalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ScrollMode) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, horizontalscrollmode: ScrollMode) -> ::windows_sys::core::HRESULT,
    pub VerticalScrollModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVerticalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ScrollMode) -> ::windows_sys::core::HRESULT,
    pub SetVerticalScrollMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, verticalscrollmode: ScrollMode) -> ::windows_sys::core::HRESULT,
    pub ZoomModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetZoomMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ZoomMode) -> ::windows_sys::core::HRESULT,
    pub SetZoomMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, zoommode: ZoomMode) -> ::windows_sys::core::HRESULT,
    pub IsDeferredScrollingEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsDeferredScrollingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDeferredScrollingEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, isdeferredscrollingenabled: bool) -> ::windows_sys::core::HRESULT,
    pub BringIntoViewOnFocusChangeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBringIntoViewOnFocusChange: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBringIntoViewOnFocusChange: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, bringintoviewonfocuschange: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TopLeftHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LeftHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopHeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReduceViewportForCoreInputViewOcclusionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalAnchorRatioProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalAnchorRatioProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanContentRenderOutsideBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCanContentRenderOutsideBounds: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanContentRenderOutsideBounds: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, cancontentrenderoutsidebounds: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewerView {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewerViewChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsIntermediate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollViewerViewChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NextView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FinalView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetQueryText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FocusOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ChooseSuggestionOnEnter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetChooseSuggestionOnEnter: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub QueryChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQueryChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQueryChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestionsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuggestionsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub QuerySubmitted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuerySubmitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuerySubmitted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuerySubmitted: usize,
    #[cfg(feature = "Foundation")]
    pub ResultSuggestionChosen: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResultSuggestionChosen: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResultSuggestionChosen: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResultSuggestionChosen: usize,
    #[cfg(feature = "Foundation")]
    pub PrepareForFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrepareForFocusOnKeyboardInput: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrepareForFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrepareForFocusOnKeyboardInput: usize,
    #[cfg(feature = "ApplicationModel_Search")]
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    SetLocalContentSuggestionSettings: usize,
}
#[repr(C)]
pub struct ISearchBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchBoxQueryChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
#[repr(C)]
pub struct ISearchBoxQuerySubmittedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
}
#[repr(C)]
pub struct ISearchBoxResultSuggestionChosenEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
}
#[repr(C)]
pub struct ISearchBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SearchHistoryEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SearchHistoryContextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusOnKeyboardInputProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChooseSuggestionOnEnterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchBoxSuggestionsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
    #[cfg(feature = "ApplicationModel_Search")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    Request: usize,
}
#[repr(C)]
pub struct ISectionsInViewChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedSections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedSections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedSections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedSections: usize,
}
#[repr(C)]
pub struct ISectionsInViewChangedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISelectionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedItems: usize,
}
#[repr(C)]
pub struct ISelectionChangedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstanceWithRemovedItemsAndAddedItems: unsafe extern "system" fn(this: *mut *mut Self, removeditems: *mut ::core::ffi::c_void, addeditems: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstanceWithRemovedItemsAndAddedItems: usize,
}
#[repr(C)]
pub struct ISemanticZoom {
    pub base__: ::windows_sys::core::IInspectable,
    pub ZoomedInView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetZoomedInView: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomedOutView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetZoomedOutView: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomedInViewActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomedInViewActive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanChangeViews: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanChangeViews: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewChangeStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewChangeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewChangeStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewChangeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ViewChangeCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewChangeCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewChangeCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewChangeCompleted: usize,
    pub ToggleActiveView: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsZoomOutButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomOutButtonEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISemanticZoomInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub SemanticZoomOwner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSemanticZoomOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsActiveView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsActiveView: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsZoomedInView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZoomedInView: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InitializeViewChange: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CompleteViewChange: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MakeVisible: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartViewChangeFrom: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartViewChangeTo: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompleteViewChangeFrom: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompleteViewChangeTo: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISemanticZoomLocation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
}
#[repr(C)]
pub struct ISemanticZoomStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ZoomedInViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomedOutViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomedInViewActiveProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanChangeViewsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsZoomOutButtonEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISemanticZoomViewChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSourceZoomedInView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSourceZoomedInView: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SourceItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSourceItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DestinationItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDestinationItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HeaderBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HeaderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetHeaderBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetHeaderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HeaderForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HeaderForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetHeaderForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetHeaderForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub IconSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    IconSource: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetIconSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetIconSource: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "Foundation")]
    pub BackClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackClick: usize,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShowIndependent: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IconSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISlider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IntermediateValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetIntermediateValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub StepFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStepFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SnapsTo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::SliderSnapsTo) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SnapsTo: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetSnapsTo: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::SliderSnapsTo) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetSnapsTo: usize,
    pub TickFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTickFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TickPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::TickPlacement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TickPlacement: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetTickPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::TickPlacement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetTickPlacement: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    pub IsDirectionReversed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDirectionReversed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsThumbToolTipEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsThumbToolTipEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Data")]
    pub ThumbToolTipValueConverter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Data"))]
    ThumbToolTipValueConverter: usize,
    #[cfg(feature = "UI_Xaml_Data")]
    pub SetThumbToolTipValueConverter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Data"))]
    SetThumbToolTipValueConverter: usize,
}
#[repr(C)]
pub struct ISlider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISliderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISliderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IntermediateValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StepFrequencyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SnapsToProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TickFrequencyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TickPlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsDirectionReversedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsThumbToolTipEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ThumbToolTipValueConverterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISliderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitButton {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Flyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Flyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetFlyout: usize,
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
    pub Click: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Click: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClick: usize,
}
#[repr(C)]
pub struct ISplitButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISplitButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitButtonClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISplitButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pane: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPane: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPaneOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPaneOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OpenPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpenPaneLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CompactPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCompactPaneLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub PanePlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SplitViewPanePlacement) -> ::windows_sys::core::HRESULT,
    pub SetPanePlacement: unsafe extern "system" fn(this: *mut *mut Self, value: SplitViewPanePlacement) -> ::windows_sys::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SplitViewDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, value: SplitViewDisplayMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PaneBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PaneBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPaneBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPaneBackground: usize,
    #[cfg(feature = "Foundation")]
    pub PaneClosing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneClosing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneClosing: usize,
    #[cfg(feature = "Foundation")]
    pub PaneClosed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneClosed: usize,
}
#[repr(C)]
pub struct ISplitView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitView3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PaneOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneOpening: usize,
    #[cfg(feature = "Foundation")]
    pub PaneOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PaneOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaneOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaneOpened: usize,
}
#[repr(C)]
pub struct ISplitViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitViewPaneClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPaneOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenPaneLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompactPaneLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PanePlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TemplateSettingsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaneBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreScrollSnapPointsRegular: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreScrollSnapPointsRegular: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanel2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanel4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Spacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanel5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundSizing) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSizing: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundSizing) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreScrollSnapPointsRegularProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanelStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanelStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStackPanelStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStyleSelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectStyle: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStyleSelectorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStyleSelectorOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectStyleCore: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwapChainBackgroundPanel {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISwapChainBackgroundPanel2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub CreateCoreIndependentInputSource: unsafe extern "system" fn(this: *mut *mut Self, devicetypes: super::super::Core::CoreInputDeviceTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateCoreIndependentInputSource: usize,
}
#[repr(C)]
pub struct ISwapChainBackgroundPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwapChainPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompositionScaleX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub CompositionScaleY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompositionScaleChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionScaleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionScaleChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionScaleChanged: usize,
    #[cfg(feature = "UI_Core")]
    pub CreateCoreIndependentInputSource: unsafe extern "system" fn(this: *mut *mut Self, devicetypes: super::super::Core::CoreInputDeviceTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateCoreIndependentInputSource: usize,
}
#[repr(C)]
pub struct ISwapChainPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwapChainPanelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompositionScaleXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompositionScaleYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLeftItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRightItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BottomItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottomItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BottomItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IconSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIconSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
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
    pub BehaviorOnInvoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SwipeBehaviorOnInvoked) -> ::windows_sys::core::HRESULT,
    pub SetBehaviorOnInvoked: unsafe extern "system" fn(this: *mut *mut Self, value: SwipeBehaviorOnInvoked) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
}
#[repr(C)]
pub struct ISwipeItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeItemInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SwipeControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BehaviorOnInvokedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeItems {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SwipeMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: SwipeMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeItemsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeItemsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymbolIcon {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Symbol) -> ::windows_sys::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut *mut Self, value: Symbol) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymbolIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithSymbol: unsafe extern "system" fn(this: *mut *mut Self, symbol: Symbol, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymbolIconSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Symbol) -> ::windows_sys::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut *mut Self, value: Symbol) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymbolIconSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymbolIconSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SymbolProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymbolIconStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SymbolProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlock {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    pub CharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub TextWrapping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub TextTrimming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextTrimming) -> ::windows_sys::core::HRESULT,
    pub SetTextTrimming: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextTrimming) -> ::windows_sys::core::HRESULT,
    pub TextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents"))]
    pub Inlines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents")))]
    Inlines: usize,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub IsTextSelectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextSelectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SelectedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentStart: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub ContentEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    ContentEnd: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub SelectionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    SelectionStart: usize,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub SelectionEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    SelectionEnd: usize,
    pub BaselineOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContextMenuOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextMenuOpening: usize,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Documents")]
    pub Select: unsafe extern "system" fn(this: *mut *mut Self, start: *mut ::core::ffi::c_void, end: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Documents"))]
    Select: usize,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlock2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColor: usize,
    pub MaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLines: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TextLineBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextLineBounds) -> ::windows_sys::core::HRESULT,
    pub SetTextLineBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextLineBounds) -> ::windows_sys::core::HRESULT,
    pub OpticalMarginAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::OpticalMarginAlignment) -> ::windows_sys::core::HRESULT,
    pub SetOpticalMarginAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::OpticalMarginAlignment) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub SetTextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlock3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlock4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub GetAlphaMask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetAlphaMask: usize,
}
#[repr(C)]
pub struct ITextBlock5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Text")]
    pub TextDecorations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::TextDecorations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    TextDecorations: usize,
    #[cfg(feature = "UI_Text")]
    pub SetTextDecorations: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::TextDecorations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetTextDecorations: usize,
}
#[repr(C)]
pub struct ITextBlock6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextTrimmed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents"))]
    pub TextHighlighters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Documents")))]
    TextHighlighters: usize,
    #[cfg(feature = "Foundation")]
    pub IsTextTrimmedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsTextTrimmedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsTextTrimmedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsTextTrimmedChanged: usize,
}
#[repr(C)]
pub struct ITextBlock7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetSelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetSelectionFlyout: usize,
    pub CopySelectionToClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlockStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextWrappingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextTrimmingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextSelectionEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlockStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionHighlightColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxLinesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextLineBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpticalMarginAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlockStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlockStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextDecorationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlockStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextTrimmedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBlockStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SelectedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSelectedText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SelectionLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectionLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectionStart: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AcceptsReturn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAcceptsReturn: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub TextWrapping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextWrapping) -> ::windows_sys::core::HRESULT,
    pub IsSpellCheckEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSpellCheckEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsTextPredictionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextPredictionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub InputScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    InputScope: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetInputScope: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetInputScope: usize,
    #[cfg(feature = "Foundation")]
    pub TextChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContextMenuOpening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextMenuOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextMenuOpening: usize,
    pub Select: unsafe extern "system" fn(this: *mut *mut Self, start: i32, length: i32) -> ::windows_sys::core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetRectFromCharacterIndex: unsafe extern "system" fn(this: *mut *mut Self, charindex: i32, trailingedge: bool, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRectFromCharacterIndex: usize,
}
#[repr(C)]
pub struct ITextBox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColor: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColor: usize,
    pub PreventKeyboardDisplayOnProgrammaticFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreventKeyboardDisplayOnProgrammaticFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Paste: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paste: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaste: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaste: usize,
}
#[repr(C)]
pub struct ITextBox3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TextCompositionStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextCompositionStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub TextCompositionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextCompositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextCompositionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextCompositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TextCompositionEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextCompositionEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextCompositionEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextCompositionEnded: usize,
    pub TextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub SetTextReadingOrder: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextReadingOrder) -> ::windows_sys::core::HRESULT,
    pub DesiredCandidateWindowAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CandidateWindowAlignment) -> ::windows_sys::core::HRESULT,
    pub SetDesiredCandidateWindowAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: CandidateWindowAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CandidateWindowBoundsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CandidateWindowBoundsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCandidateWindowBoundsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCandidateWindowBoundsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TextChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextChanging: usize,
}
#[repr(C)]
pub struct ITextBox4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetLinguisticAlternativesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetLinguisticAlternativesAsync: usize,
}
#[repr(C)]
pub struct ITextBox5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionHighlightColorWhenNotFocused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionHighlightColorWhenNotFocused: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionHighlightColorWhenNotFocused: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionHighlightColorWhenNotFocused: usize,
}
#[repr(C)]
pub struct ITextBox6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub CharacterCasing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CharacterCasing) -> ::windows_sys::core::HRESULT,
    pub SetCharacterCasing: unsafe extern "system" fn(this: *mut *mut Self, value: CharacterCasing) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderForeground: usize,
    #[cfg(feature = "Foundation")]
    pub CopyingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCopyingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCopyingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub CuttingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CuttingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCuttingToClipboard: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCuttingToClipboard: usize,
    #[cfg(feature = "Foundation")]
    pub BeforeTextChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeforeTextChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBeforeTextChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBeforeTextChanging: usize,
}
#[repr(C)]
pub struct ITextBox7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HandwritingView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHandwritingView: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHandwritingViewEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHandwritingViewEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBox8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanPasteClipboardContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanUndo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanRedo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetSelectionFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetSelectionFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ProofingMenuFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ProofingMenuFlyout: usize,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanging: usize,
    pub Undo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Redo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PasteFromClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CopySelectionToClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CutSelectionToClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ClearUndoRedoHistory: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxBeforeTextChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxSelectionChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SelectionLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AcceptsReturnProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextWrappingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSpellCheckEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextPredictionEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InputScopeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionHighlightColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreventKeyboardDisplayOnProgrammaticFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredCandidateWindowAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextReadingOrderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionHighlightColorWhenNotFocusedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterCasingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HandwritingViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHandwritingViewEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxStatics8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanPasteClipboardContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanUndoProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanRedoProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProofingMenuFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextBoxTextChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextBoxTextChangingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentChanging: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextCommandBarFlyout {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextCommandBarFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextCompositionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextCompositionEndedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextCompositionStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextControlCopyingToClipboardEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextControlCuttingToClipboardEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextControlPasteEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OldTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewTime: usize,
}
#[repr(C)]
pub struct ITimePicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClockIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetClockIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MinuteIncrement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinuteIncrement: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    #[cfg(feature = "Foundation")]
    pub SetTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTime: usize,
    #[cfg(feature = "Foundation")]
    pub TimeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimeChanged: usize,
}
#[repr(C)]
pub struct ITimePicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SelectedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectedTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SelectedTimeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTimeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectedTimeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectedTimeChanged: usize,
}
#[repr(C)]
pub struct ITimePickerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClockIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetClockIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    #[cfg(feature = "Foundation")]
    pub SetTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTime: usize,
    pub MinuteIncrement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinuteIncrement: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimePicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimePicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimePicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimePicked: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAtAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAtAsync: usize,
}
#[repr(C)]
pub struct ITimePickerFlyoutPresenter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITimePickerFlyoutPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDefaultShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerFlyoutPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultShadowEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerFlyoutStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClockIdentifierProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinuteIncrementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerSelectedValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OldTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewTime: usize,
}
#[repr(C)]
pub struct ITimePickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClockIdentifierProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinuteIncrementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimePickerValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OldTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewTime: usize,
}
#[repr(C)]
pub struct IToggleMenuFlyoutItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleMenuFlyoutItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleMenuFlyoutItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCheckedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleSplitButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCheckedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsCheckedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsCheckedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsCheckedChanged: usize,
}
#[repr(C)]
pub struct IToggleSplitButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToggleSplitButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleSplitButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleSplitButtonIsCheckedChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToggleSwitch {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOn: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOnContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOnContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffContentTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "Foundation")]
    pub Toggled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Toggled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveToggled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveToggled: usize,
}
#[repr(C)]
pub struct IToggleSwitchOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnToggled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnOnContentChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnOffContentChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnHeaderChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleSwitchStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOnProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffContentTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToolTip {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Placement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Primitives::PlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Placement: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: Primitives::PlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetPlacement: usize,
    pub PlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub TemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    TemplateSettings: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
}
#[repr(C)]
pub struct IToolTip2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PlacementRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlacementRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetPlacementRect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPlacementRect: usize,
}
#[repr(C)]
pub struct IToolTipFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToolTipService {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToolTipServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub GetPlacement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut Primitives::PlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    GetPlacement: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetPlacement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: Primitives::PlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetPlacement: usize,
    pub PlacementTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToolTipProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetToolTip: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetToolTip: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToolTipStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlacementTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToolTipStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementRectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RootNodes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RootNodes: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TreeViewSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: TreeViewSelectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectedNodes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectedNodes: usize,
    pub Expand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub Expanding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Expanding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExpanding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExpanding: usize,
    #[cfg(feature = "Foundation")]
    pub Collapsed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Collapsed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCollapsed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCollapsed: usize,
}
#[repr(C)]
pub struct ITreeView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NodeFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContainerFromNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemFromContainer: unsafe extern "system" fn(this: *mut *mut Self, container: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContainerFromItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanDragItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanDragItems: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanReorderItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanReorderItems: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplateSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemContainerStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemContainerStyleSelector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ItemContainerTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ItemContainerTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetItemContainerTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetItemContainerTransitions: usize,
    pub ItemsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DragItemsStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragItemsStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragItemsStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragItemsStarting: usize,
    #[cfg(feature = "Foundation")]
    pub DragItemsCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragItemsCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragItemsCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragItemsCompleted: usize,
}
#[repr(C)]
pub struct ITreeViewCollapsedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Node: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewCollapsedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewDragItemsCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub DropResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    DropResult: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[repr(C)]
pub struct ITreeViewDragItemsStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Data: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[repr(C)]
pub struct ITreeViewExpandingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Node: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewExpandingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub GlyphOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGlyphOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub GlyphBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    GlyphBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetGlyphBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetGlyphBrush: usize,
    pub ExpandedGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExpandedGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CollapsedGlyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCollapsedGlyph: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GlyphSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGlyphSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsExpanded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExpanded: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TreeViewItemTemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasUnrealizedChildren: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHasUnrealizedChildren: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ItemsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InvokedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GlyphOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GlyphBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExpandedGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CollapsedGlyphProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GlyphSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsExpandedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TreeViewItemTemplateSettingsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasUnrealizedChildrenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpandedGlyphVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub CollapsedGlyphVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Visibility) -> ::windows_sys::core::HRESULT,
    pub Indentation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub DragItemsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemTemplateSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewItemTemplateSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpandedGlyphVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CollapsedGlyphVisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IndentationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragItemsCountProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewList {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITreeViewListFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsExpanded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExpanded: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HasChildren: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Depth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub HasUnrealizedChildren: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHasUnrealizedChildren: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
}
#[repr(C)]
pub struct ITreeViewNodeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewNodeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsExpandedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasChildrenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITreeViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanDragItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanReorderItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerStyleSelectorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemContainerTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITwoPaneView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pane1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPane1: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pane2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPane2: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pane1Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::GridLength) -> ::windows_sys::core::HRESULT,
    pub SetPane1Length: unsafe extern "system" fn(this: *mut *mut Self, value: super::GridLength) -> ::windows_sys::core::HRESULT,
    pub Pane2Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::GridLength) -> ::windows_sys::core::HRESULT,
    pub SetPane2Length: unsafe extern "system" fn(this: *mut *mut Self, value: super::GridLength) -> ::windows_sys::core::HRESULT,
    pub PanePriority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TwoPaneViewPriority) -> ::windows_sys::core::HRESULT,
    pub SetPanePriority: unsafe extern "system" fn(this: *mut *mut Self, value: TwoPaneViewPriority) -> ::windows_sys::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TwoPaneViewMode) -> ::windows_sys::core::HRESULT,
    pub WideModeConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TwoPaneViewWideModeConfiguration) -> ::windows_sys::core::HRESULT,
    pub SetWideModeConfiguration: unsafe extern "system" fn(this: *mut *mut Self, value: TwoPaneViewWideModeConfiguration) -> ::windows_sys::core::HRESULT,
    pub TallModeConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TwoPaneViewTallModeConfiguration) -> ::windows_sys::core::HRESULT,
    pub SetTallModeConfiguration: unsafe extern "system" fn(this: *mut *mut Self, value: TwoPaneViewTallModeConfiguration) -> ::windows_sys::core::HRESULT,
    pub MinWideModeWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinWideModeWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MinTallModeHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinTallModeHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ModeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveModeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveModeChanged: usize,
}
#[repr(C)]
pub struct ITwoPaneViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITwoPaneViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pane1Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pane2Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pane1LengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pane2LengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PanePriorityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WideModeConfigurationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TallModeConfigurationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinWideModeWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinTallModeHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, oldindex: u32, newindex: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserControlFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVariableSizedWrapGrid {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetItemHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ItemWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetItemWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    pub HorizontalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub MaximumRowsOrColumns: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaximumRowsOrColumns: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVariableSizedWrapGridStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalChildrenAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalChildrenAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaximumRowsOrColumnsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowSpanProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRowSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRowSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub ColumnSpanProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetColumnSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetColumnSpan: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IViewbox {
    pub base__: ::windows_sys::core::IInspectable,
    pub Child: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    pub StretchDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StretchDirection) -> ::windows_sys::core::HRESULT,
    pub SetStretchDirection: unsafe extern "system" fn(this: *mut *mut Self, value: StretchDirection) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IViewboxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualizingPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemContainerGenerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualizingPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVirtualizingPanelOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub OnItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    OnItemsChanged: usize,
    pub OnClearChildren: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BringIndexIntoView: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualizingPanelProtected {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddInternalChild: unsafe extern "system" fn(this: *mut *mut Self, child: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertInternalChild: unsafe extern "system" fn(this: *mut *mut Self, index: i32, child: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveInternalChildRange: unsafe extern "system" fn(this: *mut *mut Self, index: i32, range: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualizingStackPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreScrollSnapPointsRegular: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreScrollSnapPointsRegular: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CleanUpVirtualizedItemEvent: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CleanUpVirtualizedItemEvent: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCleanUpVirtualizedItemEvent: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCleanUpVirtualizedItemEvent: usize,
}
#[repr(C)]
pub struct IVirtualizingStackPanelOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnCleanUpVirtualizedItem: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualizingStackPanelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreScrollSnapPointsRegularProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VirtualizationModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVirtualizationMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut VirtualizationMode) -> ::windows_sys::core::HRESULT,
    pub SetVirtualizationMode: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: VirtualizationMode) -> ::windows_sys::core::HRESULT,
    pub IsVirtualizingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsVirtualizing: unsafe extern "system" fn(this: *mut *mut Self, o: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Source: usize,
    #[cfg(feature = "Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSource: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AllowedScriptNotifyUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AllowedScriptNotifyUris: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SetAllowedScriptNotifyUris: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SetAllowedScriptNotifyUris: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "deprecated"))]
    pub DataTransferPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "deprecated")))]
    DataTransferPackage: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Navigation", feature = "deprecated"))]
    pub LoadCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Navigation", feature = "deprecated")))]
    LoadCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLoadCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLoadCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ScriptNotify: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScriptNotify: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScriptNotify: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub NavigationFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    NavigationFailed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveNavigationFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveNavigationFailed: usize,
    #[cfg(feature = "deprecated")]
    pub InvokeScript: unsafe extern "system" fn(this: *mut *mut Self, scriptname: ::windows_sys::core::HSTRING, arguments_array_size: u32, arguments: *const ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvokeScript: usize,
    #[cfg(feature = "Foundation")]
    pub Navigate: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigate: usize,
    pub NavigateToString: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanGoBack: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ContentLoading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentLoading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub DOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDOMContentLoaded: usize,
    pub GoForward: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GoBack: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CapturePreviewToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CapturePreviewToStreamAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InvokeScriptAsync: unsafe extern "system" fn(this: *mut *mut Self, scriptname: ::windows_sys::core::HSTRING, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InvokeScriptAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub CaptureSelectedContentToDataPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation")))]
    CaptureSelectedContentToDataPackageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web"))]
    pub NavigateToLocalStreamUri: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, streamresolver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web")))]
    NavigateToLocalStreamUri: usize,
    #[cfg(feature = "Foundation")]
    pub BuildLocalStreamUri: unsafe extern "system" fn(this: *mut *mut Self, contentidentifier: ::windows_sys::core::HSTRING, relativepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BuildLocalStreamUri: usize,
    pub DefaultBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub FrameContentLoading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameContentLoading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub FrameDOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameDOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub LongRunningScriptDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLongRunningScriptDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub UnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub UnviewableContentIdentified: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnviewableContentIdentified: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnviewableContentIdentified: usize,
    #[cfg(feature = "Web_Http")]
    pub NavigateWithHttpRequestMessage: unsafe extern "system" fn(this: *mut *mut Self, requestmessage: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    NavigateWithHttpRequestMessage: usize,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContainsFullScreenElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContainsFullScreenElementChanged: usize,
}
#[repr(C)]
pub struct IWebView4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExecutionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewExecutionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeferredPermissionRequests: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeferredPermissionRequests: usize,
    pub Settings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub NewWindowRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWindowRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PermissionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePermissionRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePermissionRequested: usize,
    pub AddWebAllowedObject: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, pobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeferredPermissionRequestById: unsafe extern "system" fn(this: *mut *mut Self, id: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebView5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub XYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebView6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SeparateProcessLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeparateProcessLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSeparateProcessLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSeparateProcessLost: usize,
}
#[repr(C)]
pub struct IWebView7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub WebResourceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WebResourceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWebResourceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWebResourceRequested: usize,
}
#[repr(C)]
pub struct IWebViewBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSourceName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Redraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewContentLoadingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[repr(C)]
pub struct IWebViewDOMContentLoadedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[repr(C)]
pub struct IWebViewDeferredPermissionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewPermissionType) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewFactory4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithExecutionMode: unsafe extern "system" fn(this: *mut *mut Self, executionmode: WebViewExecutionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewLongRunningScriptDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ExecutionTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionTime: usize,
    pub StopPageScriptExecution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStopPageScriptExecution: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewNavigationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub IsSuccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Web")]
    pub WebErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Web::WebErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    WebErrorStatus: usize,
}
#[repr(C)]
pub struct IWebViewNavigationFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Web")]
    pub WebErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Web::WebErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    WebErrorStatus: usize,
}
#[repr(C)]
pub struct IWebViewNavigationStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewNewWindowRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewPermissionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewPermissionType) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewPermissionState) -> ::windows_sys::core::HRESULT,
    pub Defer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewPermissionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PermissionRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewSeparateProcessLostEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWebViewSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsJavaScriptEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsJavaScriptEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsIndexedDBEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIndexedDBEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AnyScriptNotifyUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AnyScriptNotifyUri: usize,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub AllowedScriptNotifyUrisProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowedScriptNotifyUrisProperty: usize,
    #[cfg(feature = "deprecated")]
    pub DataTransferPackageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DataTransferPackageProperty: usize,
}
#[repr(C)]
pub struct IWebViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanGoBackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanGoForwardProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentTitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultBackgroundColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContainsFullScreenElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultExecutionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewExecutionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearTemporaryWebDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearTemporaryWebDataAsync: usize,
}
#[repr(C)]
pub struct IWebViewStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub XYFocusLeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewUnsupportedUriSchemeIdentifiedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewUnviewableContentIdentifiedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
}
#[repr(C)]
pub struct IWebViewUnviewableContentIdentifiedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebViewWebResourceRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Http")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Request: usize,
    #[cfg(feature = "Web_Http")]
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Response: usize,
    #[cfg(feature = "Web_Http")]
    pub SetResponse: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    SetResponse: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IWrapGrid {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetItemWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ItemHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetItemHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: Orientation) -> ::windows_sys::core::HRESULT,
    pub HorizontalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalChildrenAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub MaximumRowsOrColumns: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaximumRowsOrColumns: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWrapGridStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalChildrenAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalChildrenAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaximumRowsOrColumnsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type IconElement = *mut ::core::ffi::c_void;
pub type IconSource = *mut ::core::ffi::c_void;
pub type IconSourceElement = *mut ::core::ffi::c_void;
pub type Image = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct IncrementalLoadingTrigger(pub i32);
impl IncrementalLoadingTrigger {
    pub const None: Self = Self(0i32);
    pub const Edge: Self = Self(1i32);
}
impl ::core::marker::Copy for IncrementalLoadingTrigger {}
impl ::core::clone::Clone for IncrementalLoadingTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkCanvas = *mut ::core::ffi::c_void;
pub type InkToolbar = *mut ::core::ffi::c_void;
pub type InkToolbarBallpointPenButton = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarButtonFlyoutPlacement(pub i32);
impl InkToolbarButtonFlyoutPlacement {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
}
impl ::core::marker::Copy for InkToolbarButtonFlyoutPlacement {}
impl ::core::clone::Clone for InkToolbarButtonFlyoutPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkToolbarCustomPen = *mut ::core::ffi::c_void;
pub type InkToolbarCustomPenButton = *mut ::core::ffi::c_void;
pub type InkToolbarCustomToggleButton = *mut ::core::ffi::c_void;
pub type InkToolbarCustomToolButton = *mut ::core::ffi::c_void;
pub type InkToolbarEraserButton = *mut ::core::ffi::c_void;
pub type InkToolbarFlyoutItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarFlyoutItemKind(pub i32);
impl InkToolbarFlyoutItemKind {
    pub const Simple: Self = Self(0i32);
    pub const Radio: Self = Self(1i32);
    pub const Check: Self = Self(2i32);
    pub const RadioCheck: Self = Self(3i32);
}
impl ::core::marker::Copy for InkToolbarFlyoutItemKind {}
impl ::core::clone::Clone for InkToolbarFlyoutItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkToolbarHighlighterButton = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarInitialControls(pub i32);
impl InkToolbarInitialControls {
    pub const All: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const PensOnly: Self = Self(2i32);
    pub const AllExceptPens: Self = Self(3i32);
}
impl ::core::marker::Copy for InkToolbarInitialControls {}
impl ::core::clone::Clone for InkToolbarInitialControls {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkToolbarIsStencilButtonCheckedChangedEventArgs = *mut ::core::ffi::c_void;
pub type InkToolbarMenuButton = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarMenuKind(pub i32);
impl InkToolbarMenuKind {
    pub const Stencil: Self = Self(0i32);
}
impl ::core::marker::Copy for InkToolbarMenuKind {}
impl ::core::clone::Clone for InkToolbarMenuKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkToolbarPenButton = *mut ::core::ffi::c_void;
pub type InkToolbarPenConfigurationControl = *mut ::core::ffi::c_void;
pub type InkToolbarPencilButton = *mut ::core::ffi::c_void;
pub type InkToolbarRulerButton = *mut ::core::ffi::c_void;
pub type InkToolbarStencilButton = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarStencilKind(pub i32);
impl InkToolbarStencilKind {
    pub const Ruler: Self = Self(0i32);
    pub const Protractor: Self = Self(1i32);
}
impl ::core::marker::Copy for InkToolbarStencilKind {}
impl ::core::clone::Clone for InkToolbarStencilKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarToggle(pub i32);
impl InkToolbarToggle {
    pub const Ruler: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for InkToolbarToggle {}
impl ::core::clone::Clone for InkToolbarToggle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkToolbarToggleButton = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct InkToolbarTool(pub i32);
impl InkToolbarTool {
    pub const BallpointPen: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
    pub const Highlighter: Self = Self(2i32);
    pub const Eraser: Self = Self(3i32);
    pub const CustomPen: Self = Self(4i32);
    pub const CustomTool: Self = Self(5i32);
}
impl ::core::marker::Copy for InkToolbarTool {}
impl ::core::clone::Clone for InkToolbarTool {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkToolbarToolButton = *mut ::core::ffi::c_void;
pub type IsTextTrimmedChangedEventArgs = *mut ::core::ffi::c_void;
pub type ItemClickEventArgs = *mut ::core::ffi::c_void;
pub type ItemClickEventHandler = *mut ::core::ffi::c_void;
pub type ItemCollection = *mut ::core::ffi::c_void;
pub type ItemContainerGenerator = *mut ::core::ffi::c_void;
pub type ItemsControl = *mut ::core::ffi::c_void;
pub type ItemsPanelTemplate = *mut ::core::ffi::c_void;
pub type ItemsPickedEventArgs = *mut ::core::ffi::c_void;
pub type ItemsPresenter = *mut ::core::ffi::c_void;
pub type ItemsStackPanel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ItemsUpdatingScrollMode(pub i32);
impl ItemsUpdatingScrollMode {
    pub const KeepItemsInView: Self = Self(0i32);
    pub const KeepScrollOffset: Self = Self(1i32);
    pub const KeepLastItemInView: Self = Self(2i32);
}
impl ::core::marker::Copy for ItemsUpdatingScrollMode {}
impl ::core::clone::Clone for ItemsUpdatingScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ItemsWrapGrid = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct LightDismissOverlayMode(pub i32);
impl LightDismissOverlayMode {
    pub const Auto: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
}
impl ::core::marker::Copy for LightDismissOverlayMode {}
impl ::core::clone::Clone for LightDismissOverlayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ListBox = *mut ::core::ffi::c_void;
pub type ListBoxItem = *mut ::core::ffi::c_void;
pub type ListPickerFlyout = *mut ::core::ffi::c_void;
pub type ListPickerFlyoutPresenter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ListPickerFlyoutSelectionMode(pub i32);
impl ListPickerFlyoutSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for ListPickerFlyoutSelectionMode {}
impl ::core::clone::Clone for ListPickerFlyoutSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ListView = *mut ::core::ffi::c_void;
pub type ListViewBase = *mut ::core::ffi::c_void;
pub type ListViewBaseHeaderItem = *mut ::core::ffi::c_void;
pub type ListViewHeaderItem = *mut ::core::ffi::c_void;
pub type ListViewItem = *mut ::core::ffi::c_void;
pub type ListViewItemToKeyHandler = *mut ::core::ffi::c_void;
pub type ListViewKeyToItemHandler = *mut ::core::ffi::c_void;
pub type ListViewPersistenceHelper = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ListViewReorderMode(pub i32);
impl ListViewReorderMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewReorderMode {}
impl ::core::clone::Clone for ListViewReorderMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ListViewSelectionMode(pub i32);
impl ListViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
    pub const Extended: Self = Self(3i32);
}
impl ::core::marker::Copy for ListViewSelectionMode {}
impl ::core::clone::Clone for ListViewSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaElement = *mut ::core::ffi::c_void;
pub type MediaPlayerElement = *mut ::core::ffi::c_void;
pub type MediaPlayerPresenter = *mut ::core::ffi::c_void;
pub type MediaTransportControls = *mut ::core::ffi::c_void;
pub type MediaTransportControlsHelper = *mut ::core::ffi::c_void;
pub type MenuBar = *mut ::core::ffi::c_void;
pub type MenuBarItem = *mut ::core::ffi::c_void;
pub type MenuBarItemFlyout = *mut ::core::ffi::c_void;
pub type MenuFlyout = *mut ::core::ffi::c_void;
pub type MenuFlyoutItem = *mut ::core::ffi::c_void;
pub type MenuFlyoutItemBase = *mut ::core::ffi::c_void;
pub type MenuFlyoutPresenter = *mut ::core::ffi::c_void;
pub type MenuFlyoutSeparator = *mut ::core::ffi::c_void;
pub type MenuFlyoutSubItem = *mut ::core::ffi::c_void;
pub type NavigationView = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct NavigationViewBackButtonVisible(pub i32);
impl NavigationViewBackButtonVisible {
    pub const Collapsed: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationViewBackButtonVisible {}
impl ::core::clone::Clone for NavigationViewBackButtonVisible {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationViewBackRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct NavigationViewDisplayMode(pub i32);
impl NavigationViewDisplayMode {
    pub const Minimal: Self = Self(0i32);
    pub const Compact: Self = Self(1i32);
    pub const Expanded: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationViewDisplayMode {}
impl ::core::clone::Clone for NavigationViewDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationViewDisplayModeChangedEventArgs = *mut ::core::ffi::c_void;
pub type NavigationViewItem = *mut ::core::ffi::c_void;
pub type NavigationViewItemBase = *mut ::core::ffi::c_void;
pub type NavigationViewItemHeader = *mut ::core::ffi::c_void;
pub type NavigationViewItemInvokedEventArgs = *mut ::core::ffi::c_void;
pub type NavigationViewItemSeparator = *mut ::core::ffi::c_void;
pub type NavigationViewList = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct NavigationViewOverflowLabelMode(pub i32);
impl NavigationViewOverflowLabelMode {
    pub const MoreLabel: Self = Self(0i32);
    pub const NoLabel: Self = Self(1i32);
}
impl ::core::marker::Copy for NavigationViewOverflowLabelMode {}
impl ::core::clone::Clone for NavigationViewOverflowLabelMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationViewPaneClosingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct NavigationViewPaneDisplayMode(pub i32);
impl NavigationViewPaneDisplayMode {
    pub const Auto: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const LeftCompact: Self = Self(3i32);
    pub const LeftMinimal: Self = Self(4i32);
}
impl ::core::marker::Copy for NavigationViewPaneDisplayMode {}
impl ::core::clone::Clone for NavigationViewPaneDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationViewSelectionChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct NavigationViewSelectionFollowsFocus(pub i32);
impl NavigationViewSelectionFollowsFocus {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for NavigationViewSelectionFollowsFocus {}
impl ::core::clone::Clone for NavigationViewSelectionFollowsFocus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct NavigationViewShoulderNavigationEnabled(pub i32);
impl NavigationViewShoulderNavigationEnabled {
    pub const WhenSelectionFollowsFocus: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationViewShoulderNavigationEnabled {}
impl ::core::clone::Clone for NavigationViewShoulderNavigationEnabled {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationViewTemplateSettings = *mut ::core::ffi::c_void;
pub type NotifyEventArgs = *mut ::core::ffi::c_void;
pub type NotifyEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct Orientation(pub i32);
impl Orientation {
    pub const Vertical: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
}
impl ::core::marker::Copy for Orientation {}
impl ::core::clone::Clone for Orientation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Page = *mut ::core::ffi::c_void;
pub type Panel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct PanelScrollingDirection(pub i32);
impl PanelScrollingDirection {
    pub const None: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
    pub const Backward: Self = Self(2i32);
}
impl ::core::marker::Copy for PanelScrollingDirection {}
impl ::core::clone::Clone for PanelScrollingDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ParallaxSourceOffsetKind(pub i32);
impl ParallaxSourceOffsetKind {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for ParallaxSourceOffsetKind {}
impl ::core::clone::Clone for ParallaxSourceOffsetKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ParallaxView = *mut ::core::ffi::c_void;
pub type PasswordBox = *mut ::core::ffi::c_void;
pub type PasswordBoxPasswordChangingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct PasswordRevealMode(pub i32);
impl PasswordRevealMode {
    pub const Peek: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
}
impl ::core::marker::Copy for PasswordRevealMode {}
impl ::core::clone::Clone for PasswordRevealMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PathIcon = *mut ::core::ffi::c_void;
pub type PathIconSource = *mut ::core::ffi::c_void;
pub type PersonPicture = *mut ::core::ffi::c_void;
pub type PickerConfirmedEventArgs = *mut ::core::ffi::c_void;
pub type PickerFlyout = *mut ::core::ffi::c_void;
pub type PickerFlyoutPresenter = *mut ::core::ffi::c_void;
pub type Pivot = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct PivotHeaderFocusVisualPlacement(pub i32);
impl PivotHeaderFocusVisualPlacement {
    pub const ItemHeaders: Self = Self(0i32);
    pub const SelectedItemHeader: Self = Self(1i32);
}
impl ::core::marker::Copy for PivotHeaderFocusVisualPlacement {}
impl ::core::clone::Clone for PivotHeaderFocusVisualPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PivotItem = *mut ::core::ffi::c_void;
pub type PivotItemEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct PivotSlideInAnimationGroup(pub i32);
impl PivotSlideInAnimationGroup {
    pub const Default: Self = Self(0i32);
    pub const GroupOne: Self = Self(1i32);
    pub const GroupTwo: Self = Self(2i32);
    pub const GroupThree: Self = Self(3i32);
}
impl ::core::marker::Copy for PivotSlideInAnimationGroup {}
impl ::core::clone::Clone for PivotSlideInAnimationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProgressBar = *mut ::core::ffi::c_void;
pub type ProgressRing = *mut ::core::ffi::c_void;
pub type RadioButton = *mut ::core::ffi::c_void;
pub type RatingControl = *mut ::core::ffi::c_void;
pub type RatingItemFontInfo = *mut ::core::ffi::c_void;
pub type RatingItemImageInfo = *mut ::core::ffi::c_void;
pub type RatingItemInfo = *mut ::core::ffi::c_void;
pub type RefreshContainer = *mut ::core::ffi::c_void;
pub type RefreshInteractionRatioChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct RefreshPullDirection(pub i32);
impl RefreshPullDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const TopToBottom: Self = Self(1i32);
    pub const RightToLeft: Self = Self(2i32);
    pub const BottomToTop: Self = Self(3i32);
}
impl ::core::marker::Copy for RefreshPullDirection {}
impl ::core::clone::Clone for RefreshPullDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RefreshRequestedEventArgs = *mut ::core::ffi::c_void;
pub type RefreshStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type RefreshVisualizer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct RefreshVisualizerOrientation(pub i32);
impl RefreshVisualizerOrientation {
    pub const Auto: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Rotate90DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotate270DegreesCounterclockwise: Self = Self(3i32);
}
impl ::core::marker::Copy for RefreshVisualizerOrientation {}
impl ::core::clone::Clone for RefreshVisualizerOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct RefreshVisualizerState(pub i32);
impl RefreshVisualizerState {
    pub const Idle: Self = Self(0i32);
    pub const Peeking: Self = Self(1i32);
    pub const Interacting: Self = Self(2i32);
    pub const Pending: Self = Self(3i32);
    pub const Refreshing: Self = Self(4i32);
}
impl ::core::marker::Copy for RefreshVisualizerState {}
impl ::core::clone::Clone for RefreshVisualizerState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RelativePanel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct RequiresPointer(pub i32);
impl RequiresPointer {
    pub const Never: Self = Self(0i32);
    pub const WhenEngaged: Self = Self(1i32);
    pub const WhenFocused: Self = Self(2i32);
}
impl ::core::marker::Copy for RequiresPointer {}
impl ::core::clone::Clone for RequiresPointer {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RichEditBox = *mut ::core::ffi::c_void;
pub type RichEditBoxSelectionChangingEventArgs = *mut ::core::ffi::c_void;
pub type RichEditBoxTextChangingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct RichEditClipboardFormat(pub i32);
impl RichEditClipboardFormat {
    pub const AllFormats: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for RichEditClipboardFormat {}
impl ::core::clone::Clone for RichEditClipboardFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RichTextBlock = *mut ::core::ffi::c_void;
pub type RichTextBlockOverflow = *mut ::core::ffi::c_void;
pub type RowDefinition = *mut ::core::ffi::c_void;
pub type RowDefinitionCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ScrollBarVisibility(pub i32);
impl ScrollBarVisibility {
    pub const Disabled: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
    pub const Visible: Self = Self(3i32);
}
impl ::core::marker::Copy for ScrollBarVisibility {}
impl ::core::clone::Clone for ScrollBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ScrollContentPresenter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ScrollIntoViewAlignment(pub i32);
impl ScrollIntoViewAlignment {
    pub const Default: Self = Self(0i32);
    pub const Leading: Self = Self(1i32);
}
impl ::core::marker::Copy for ScrollIntoViewAlignment {}
impl ::core::clone::Clone for ScrollIntoViewAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ScrollMode(pub i32);
impl ScrollMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for ScrollMode {}
impl ::core::clone::Clone for ScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ScrollViewer = *mut ::core::ffi::c_void;
pub type ScrollViewerView = *mut ::core::ffi::c_void;
pub type ScrollViewerViewChangedEventArgs = *mut ::core::ffi::c_void;
pub type ScrollViewerViewChangingEventArgs = *mut ::core::ffi::c_void;
pub type SearchBox = *mut ::core::ffi::c_void;
pub type SearchBoxQueryChangedEventArgs = *mut ::core::ffi::c_void;
pub type SearchBoxQuerySubmittedEventArgs = *mut ::core::ffi::c_void;
pub type SearchBoxResultSuggestionChosenEventArgs = *mut ::core::ffi::c_void;
pub type SearchBoxSuggestionsRequestedEventArgs = *mut ::core::ffi::c_void;
pub type SectionsInViewChangedEventArgs = *mut ::core::ffi::c_void;
pub type SectionsInViewChangedEventHandler = *mut ::core::ffi::c_void;
pub type SelectionChangedEventArgs = *mut ::core::ffi::c_void;
pub type SelectionChangedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct SelectionMode(pub i32);
impl SelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl ::core::marker::Copy for SelectionMode {}
impl ::core::clone::Clone for SelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SemanticZoom = *mut ::core::ffi::c_void;
pub type SemanticZoomLocation = *mut ::core::ffi::c_void;
pub type SemanticZoomViewChangedEventArgs = *mut ::core::ffi::c_void;
pub type SemanticZoomViewChangedEventHandler = *mut ::core::ffi::c_void;
pub type SettingsFlyout = *mut ::core::ffi::c_void;
pub type Slider = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct SnapPointsType(pub i32);
impl SnapPointsType {
    pub const None: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Mandatory: Self = Self(2i32);
    pub const OptionalSingle: Self = Self(3i32);
    pub const MandatorySingle: Self = Self(4i32);
}
impl ::core::marker::Copy for SnapPointsType {}
impl ::core::clone::Clone for SnapPointsType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SplitButton = *mut ::core::ffi::c_void;
pub type SplitButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type SplitButtonClickEventArgs = *mut ::core::ffi::c_void;
pub type SplitView = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct SplitViewDisplayMode(pub i32);
impl SplitViewDisplayMode {
    pub const Overlay: Self = Self(0i32);
    pub const Inline: Self = Self(1i32);
    pub const CompactOverlay: Self = Self(2i32);
    pub const CompactInline: Self = Self(3i32);
}
impl ::core::marker::Copy for SplitViewDisplayMode {}
impl ::core::clone::Clone for SplitViewDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SplitViewPaneClosingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct SplitViewPanePlacement(pub i32);
impl SplitViewPanePlacement {
    pub const Left: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
}
impl ::core::marker::Copy for SplitViewPanePlacement {}
impl ::core::clone::Clone for SplitViewPanePlacement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StackPanel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct StretchDirection(pub i32);
impl StretchDirection {
    pub const UpOnly: Self = Self(0i32);
    pub const DownOnly: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
}
impl ::core::marker::Copy for StretchDirection {}
impl ::core::clone::Clone for StretchDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StyleSelector = *mut ::core::ffi::c_void;
pub type SwapChainBackgroundPanel = *mut ::core::ffi::c_void;
pub type SwapChainPanel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct SwipeBehaviorOnInvoked(pub i32);
impl SwipeBehaviorOnInvoked {
    pub const Auto: Self = Self(0i32);
    pub const Close: Self = Self(1i32);
    pub const RemainOpen: Self = Self(2i32);
}
impl ::core::marker::Copy for SwipeBehaviorOnInvoked {}
impl ::core::clone::Clone for SwipeBehaviorOnInvoked {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SwipeControl = *mut ::core::ffi::c_void;
pub type SwipeItem = *mut ::core::ffi::c_void;
pub type SwipeItemInvokedEventArgs = *mut ::core::ffi::c_void;
pub type SwipeItems = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct SwipeMode(pub i32);
impl SwipeMode {
    pub const Reveal: Self = Self(0i32);
    pub const Execute: Self = Self(1i32);
}
impl ::core::marker::Copy for SwipeMode {}
impl ::core::clone::Clone for SwipeMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct Symbol(pub i32);
impl Symbol {
    pub const Previous: Self = Self(57600i32);
    pub const Next: Self = Self(57601i32);
    pub const Play: Self = Self(57602i32);
    pub const Pause: Self = Self(57603i32);
    pub const Edit: Self = Self(57604i32);
    pub const Save: Self = Self(57605i32);
    pub const Clear: Self = Self(57606i32);
    pub const Delete: Self = Self(57607i32);
    pub const Remove: Self = Self(57608i32);
    pub const Add: Self = Self(57609i32);
    pub const Cancel: Self = Self(57610i32);
    pub const Accept: Self = Self(57611i32);
    pub const More: Self = Self(57612i32);
    pub const Redo: Self = Self(57613i32);
    pub const Undo: Self = Self(57614i32);
    pub const Home: Self = Self(57615i32);
    pub const Up: Self = Self(57616i32);
    pub const Forward: Self = Self(57617i32);
    pub const Back: Self = Self(57618i32);
    pub const Favorite: Self = Self(57619i32);
    pub const Camera: Self = Self(57620i32);
    pub const Setting: Self = Self(57621i32);
    pub const Video: Self = Self(57622i32);
    pub const Sync: Self = Self(57623i32);
    pub const Download: Self = Self(57624i32);
    pub const Mail: Self = Self(57625i32);
    pub const Find: Self = Self(57626i32);
    pub const Help: Self = Self(57627i32);
    pub const Upload: Self = Self(57628i32);
    pub const Emoji: Self = Self(57629i32);
    pub const TwoPage: Self = Self(57630i32);
    pub const LeaveChat: Self = Self(57631i32);
    pub const MailForward: Self = Self(57632i32);
    pub const Clock: Self = Self(57633i32);
    pub const Send: Self = Self(57634i32);
    pub const Crop: Self = Self(57635i32);
    pub const RotateCamera: Self = Self(57636i32);
    pub const People: Self = Self(57637i32);
    pub const OpenPane: Self = Self(57638i32);
    pub const ClosePane: Self = Self(57639i32);
    pub const World: Self = Self(57640i32);
    pub const Flag: Self = Self(57641i32);
    pub const PreviewLink: Self = Self(57642i32);
    pub const Globe: Self = Self(57643i32);
    pub const Trim: Self = Self(57644i32);
    pub const AttachCamera: Self = Self(57645i32);
    pub const ZoomIn: Self = Self(57646i32);
    pub const Bookmarks: Self = Self(57647i32);
    pub const Document: Self = Self(57648i32);
    pub const ProtectedDocument: Self = Self(57649i32);
    pub const Page: Self = Self(57650i32);
    pub const Bullets: Self = Self(57651i32);
    pub const Comment: Self = Self(57652i32);
    pub const MailFilled: Self = Self(57653i32);
    pub const ContactInfo: Self = Self(57654i32);
    pub const HangUp: Self = Self(57655i32);
    pub const ViewAll: Self = Self(57656i32);
    pub const MapPin: Self = Self(57657i32);
    pub const Phone: Self = Self(57658i32);
    pub const VideoChat: Self = Self(57659i32);
    pub const Switch: Self = Self(57660i32);
    pub const Contact: Self = Self(57661i32);
    pub const Rename: Self = Self(57662i32);
    pub const Pin: Self = Self(57665i32);
    pub const MusicInfo: Self = Self(57666i32);
    pub const Go: Self = Self(57667i32);
    pub const Keyboard: Self = Self(57668i32);
    pub const DockLeft: Self = Self(57669i32);
    pub const DockRight: Self = Self(57670i32);
    pub const DockBottom: Self = Self(57671i32);
    pub const Remote: Self = Self(57672i32);
    pub const Refresh: Self = Self(57673i32);
    pub const Rotate: Self = Self(57674i32);
    pub const Shuffle: Self = Self(57675i32);
    pub const List: Self = Self(57676i32);
    pub const Shop: Self = Self(57677i32);
    pub const SelectAll: Self = Self(57678i32);
    pub const Orientation: Self = Self(57679i32);
    pub const Import: Self = Self(57680i32);
    pub const ImportAll: Self = Self(57681i32);
    pub const BrowsePhotos: Self = Self(57685i32);
    pub const WebCam: Self = Self(57686i32);
    pub const Pictures: Self = Self(57688i32);
    pub const SaveLocal: Self = Self(57689i32);
    pub const Caption: Self = Self(57690i32);
    pub const Stop: Self = Self(57691i32);
    pub const ShowResults: Self = Self(57692i32);
    pub const Volume: Self = Self(57693i32);
    pub const Repair: Self = Self(57694i32);
    pub const Message: Self = Self(57695i32);
    pub const Page2: Self = Self(57696i32);
    pub const CalendarDay: Self = Self(57697i32);
    pub const CalendarWeek: Self = Self(57698i32);
    pub const Calendar: Self = Self(57699i32);
    pub const Character: Self = Self(57700i32);
    pub const MailReplyAll: Self = Self(57701i32);
    pub const Read: Self = Self(57702i32);
    pub const Link: Self = Self(57703i32);
    pub const Account: Self = Self(57704i32);
    pub const ShowBcc: Self = Self(57705i32);
    pub const HideBcc: Self = Self(57706i32);
    pub const Cut: Self = Self(57707i32);
    pub const Attach: Self = Self(57708i32);
    pub const Paste: Self = Self(57709i32);
    pub const Filter: Self = Self(57710i32);
    pub const Copy: Self = Self(57711i32);
    pub const Emoji2: Self = Self(57712i32);
    pub const Important: Self = Self(57713i32);
    pub const MailReply: Self = Self(57714i32);
    pub const SlideShow: Self = Self(57715i32);
    pub const Sort: Self = Self(57716i32);
    pub const Manage: Self = Self(57720i32);
    pub const AllApps: Self = Self(57721i32);
    pub const DisconnectDrive: Self = Self(57722i32);
    pub const MapDrive: Self = Self(57723i32);
    pub const NewWindow: Self = Self(57724i32);
    pub const OpenWith: Self = Self(57725i32);
    pub const ContactPresence: Self = Self(57729i32);
    pub const Priority: Self = Self(57730i32);
    pub const GoToToday: Self = Self(57732i32);
    pub const Font: Self = Self(57733i32);
    pub const FontColor: Self = Self(57734i32);
    pub const Contact2: Self = Self(57735i32);
    pub const Folder: Self = Self(57736i32);
    pub const Audio: Self = Self(57737i32);
    pub const Placeholder: Self = Self(57738i32);
    pub const View: Self = Self(57739i32);
    pub const SetLockScreen: Self = Self(57740i32);
    pub const SetTile: Self = Self(57741i32);
    pub const ClosedCaption: Self = Self(57744i32);
    pub const StopSlideShow: Self = Self(57745i32);
    pub const Permissions: Self = Self(57746i32);
    pub const Highlight: Self = Self(57747i32);
    pub const DisableUpdates: Self = Self(57748i32);
    pub const UnFavorite: Self = Self(57749i32);
    pub const UnPin: Self = Self(57750i32);
    pub const OpenLocal: Self = Self(57751i32);
    pub const Mute: Self = Self(57752i32);
    pub const Italic: Self = Self(57753i32);
    pub const Underline: Self = Self(57754i32);
    pub const Bold: Self = Self(57755i32);
    pub const MoveToFolder: Self = Self(57756i32);
    pub const LikeDislike: Self = Self(57757i32);
    pub const Dislike: Self = Self(57758i32);
    pub const Like: Self = Self(57759i32);
    pub const AlignRight: Self = Self(57760i32);
    pub const AlignCenter: Self = Self(57761i32);
    pub const AlignLeft: Self = Self(57762i32);
    pub const Zoom: Self = Self(57763i32);
    pub const ZoomOut: Self = Self(57764i32);
    pub const OpenFile: Self = Self(57765i32);
    pub const OtherUser: Self = Self(57766i32);
    pub const Admin: Self = Self(57767i32);
    pub const Street: Self = Self(57795i32);
    pub const Map: Self = Self(57796i32);
    pub const ClearSelection: Self = Self(57797i32);
    pub const FontDecrease: Self = Self(57798i32);
    pub const FontIncrease: Self = Self(57799i32);
    pub const FontSize: Self = Self(57800i32);
    pub const CellPhone: Self = Self(57801i32);
    pub const ReShare: Self = Self(57802i32);
    pub const Tag: Self = Self(57803i32);
    pub const RepeatOne: Self = Self(57804i32);
    pub const RepeatAll: Self = Self(57805i32);
    pub const OutlineStar: Self = Self(57806i32);
    pub const SolidStar: Self = Self(57807i32);
    pub const Calculator: Self = Self(57808i32);
    pub const Directions: Self = Self(57809i32);
    pub const Target: Self = Self(57810i32);
    pub const Library: Self = Self(57811i32);
    pub const PhoneBook: Self = Self(57812i32);
    pub const Memo: Self = Self(57813i32);
    pub const Microphone: Self = Self(57814i32);
    pub const PostUpdate: Self = Self(57815i32);
    pub const BackToWindow: Self = Self(57816i32);
    pub const FullScreen: Self = Self(57817i32);
    pub const NewFolder: Self = Self(57818i32);
    pub const CalendarReply: Self = Self(57819i32);
    pub const UnSyncFolder: Self = Self(57821i32);
    pub const ReportHacked: Self = Self(57822i32);
    pub const SyncFolder: Self = Self(57823i32);
    pub const BlockContact: Self = Self(57824i32);
    pub const SwitchApps: Self = Self(57825i32);
    pub const AddFriend: Self = Self(57826i32);
    pub const TouchPointer: Self = Self(57827i32);
    pub const GoToStart: Self = Self(57828i32);
    pub const ZeroBars: Self = Self(57829i32);
    pub const OneBar: Self = Self(57830i32);
    pub const TwoBars: Self = Self(57831i32);
    pub const ThreeBars: Self = Self(57832i32);
    pub const FourBars: Self = Self(57833i32);
    pub const Scan: Self = Self(58004i32);
    pub const Preview: Self = Self(58005i32);
    pub const GlobalNavigationButton: Self = Self(59136i32);
    pub const Share: Self = Self(59181i32);
    pub const Print: Self = Self(59209i32);
    pub const XboxOneConsole: Self = Self(59792i32);
}
impl ::core::marker::Copy for Symbol {}
impl ::core::clone::Clone for Symbol {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SymbolIcon = *mut ::core::ffi::c_void;
pub type SymbolIconSource = *mut ::core::ffi::c_void;
pub type TextBlock = *mut ::core::ffi::c_void;
pub type TextBox = *mut ::core::ffi::c_void;
pub type TextBoxBeforeTextChangingEventArgs = *mut ::core::ffi::c_void;
pub type TextBoxSelectionChangingEventArgs = *mut ::core::ffi::c_void;
pub type TextBoxTextChangingEventArgs = *mut ::core::ffi::c_void;
pub type TextChangedEventArgs = *mut ::core::ffi::c_void;
pub type TextChangedEventHandler = *mut ::core::ffi::c_void;
pub type TextCommandBarFlyout = *mut ::core::ffi::c_void;
pub type TextCompositionChangedEventArgs = *mut ::core::ffi::c_void;
pub type TextCompositionEndedEventArgs = *mut ::core::ffi::c_void;
pub type TextCompositionStartedEventArgs = *mut ::core::ffi::c_void;
pub type TextControlCopyingToClipboardEventArgs = *mut ::core::ffi::c_void;
pub type TextControlCuttingToClipboardEventArgs = *mut ::core::ffi::c_void;
pub type TextControlPasteEventArgs = *mut ::core::ffi::c_void;
pub type TextControlPasteEventHandler = *mut ::core::ffi::c_void;
pub type TimePickedEventArgs = *mut ::core::ffi::c_void;
pub type TimePicker = *mut ::core::ffi::c_void;
pub type TimePickerFlyout = *mut ::core::ffi::c_void;
pub type TimePickerFlyoutPresenter = *mut ::core::ffi::c_void;
pub type TimePickerSelectedValueChangedEventArgs = *mut ::core::ffi::c_void;
pub type TimePickerValueChangedEventArgs = *mut ::core::ffi::c_void;
pub type ToggleMenuFlyoutItem = *mut ::core::ffi::c_void;
pub type ToggleSplitButton = *mut ::core::ffi::c_void;
pub type ToggleSplitButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type ToggleSplitButtonIsCheckedChangedEventArgs = *mut ::core::ffi::c_void;
pub type ToggleSwitch = *mut ::core::ffi::c_void;
pub type ToolTip = *mut ::core::ffi::c_void;
pub type ToolTipService = *mut ::core::ffi::c_void;
pub type TreeView = *mut ::core::ffi::c_void;
pub type TreeViewCollapsedEventArgs = *mut ::core::ffi::c_void;
pub type TreeViewDragItemsCompletedEventArgs = *mut ::core::ffi::c_void;
pub type TreeViewDragItemsStartingEventArgs = *mut ::core::ffi::c_void;
pub type TreeViewExpandingEventArgs = *mut ::core::ffi::c_void;
pub type TreeViewItem = *mut ::core::ffi::c_void;
pub type TreeViewItemInvokedEventArgs = *mut ::core::ffi::c_void;
pub type TreeViewItemTemplateSettings = *mut ::core::ffi::c_void;
pub type TreeViewList = *mut ::core::ffi::c_void;
pub type TreeViewNode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct TreeViewSelectionMode(pub i32);
impl TreeViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for TreeViewSelectionMode {}
impl ::core::clone::Clone for TreeViewSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TwoPaneView = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct TwoPaneViewMode(pub i32);
impl TwoPaneViewMode {
    pub const SinglePane: Self = Self(0i32);
    pub const Wide: Self = Self(1i32);
    pub const Tall: Self = Self(2i32);
}
impl ::core::marker::Copy for TwoPaneViewMode {}
impl ::core::clone::Clone for TwoPaneViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct TwoPaneViewPriority(pub i32);
impl TwoPaneViewPriority {
    pub const Pane1: Self = Self(0i32);
    pub const Pane2: Self = Self(1i32);
}
impl ::core::marker::Copy for TwoPaneViewPriority {}
impl ::core::clone::Clone for TwoPaneViewPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct TwoPaneViewTallModeConfiguration(pub i32);
impl TwoPaneViewTallModeConfiguration {
    pub const SinglePane: Self = Self(0i32);
    pub const TopBottom: Self = Self(1i32);
    pub const BottomTop: Self = Self(2i32);
}
impl ::core::marker::Copy for TwoPaneViewTallModeConfiguration {}
impl ::core::clone::Clone for TwoPaneViewTallModeConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct TwoPaneViewWideModeConfiguration(pub i32);
impl TwoPaneViewWideModeConfiguration {
    pub const SinglePane: Self = Self(0i32);
    pub const LeftRight: Self = Self(1i32);
    pub const RightLeft: Self = Self(2i32);
}
impl ::core::marker::Copy for TwoPaneViewWideModeConfiguration {}
impl ::core::clone::Clone for TwoPaneViewWideModeConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UIElementCollection = *mut ::core::ffi::c_void;
pub type UserControl = *mut ::core::ffi::c_void;
pub type VariableSizedWrapGrid = *mut ::core::ffi::c_void;
pub type Viewbox = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct VirtualizationMode(pub i32);
impl VirtualizationMode {
    pub const Standard: Self = Self(0i32);
    pub const Recycling: Self = Self(1i32);
}
impl ::core::marker::Copy for VirtualizationMode {}
impl ::core::clone::Clone for VirtualizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VirtualizingPanel = *mut ::core::ffi::c_void;
pub type VirtualizingStackPanel = *mut ::core::ffi::c_void;
pub type WebView = *mut ::core::ffi::c_void;
pub type WebViewBrush = *mut ::core::ffi::c_void;
pub type WebViewContentLoadingEventArgs = *mut ::core::ffi::c_void;
pub type WebViewDOMContentLoadedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewDeferredPermissionRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct WebViewExecutionMode(pub i32);
impl WebViewExecutionMode {
    pub const SameThread: Self = Self(0i32);
    pub const SeparateThread: Self = Self(1i32);
    pub const SeparateProcess: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewExecutionMode {}
impl ::core::clone::Clone for WebViewExecutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebViewLongRunningScriptDetectedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewNavigationCompletedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewNavigationFailedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewNavigationFailedEventHandler = *mut ::core::ffi::c_void;
pub type WebViewNavigationStartingEventArgs = *mut ::core::ffi::c_void;
pub type WebViewNewWindowRequestedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewPermissionRequest = *mut ::core::ffi::c_void;
pub type WebViewPermissionRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct WebViewPermissionState(pub i32);
impl WebViewPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl ::core::marker::Copy for WebViewPermissionState {}
impl ::core::clone::Clone for WebViewPermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct WebViewPermissionType(pub i32);
impl WebViewPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl ::core::marker::Copy for WebViewPermissionType {}
impl ::core::clone::Clone for WebViewPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebViewSeparateProcessLostEventArgs = *mut ::core::ffi::c_void;
pub type WebViewSettings = *mut ::core::ffi::c_void;
pub type WebViewUnsupportedUriSchemeIdentifiedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewUnviewableContentIdentifiedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewWebResourceRequestedEventArgs = *mut ::core::ffi::c_void;
pub type WrapGrid = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls\"`*"]
#[repr(transparent)]
pub struct ZoomMode(pub i32);
impl ZoomMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for ZoomMode {}
impl ::core::clone::Clone for ZoomMode {
    fn clone(&self) -> Self {
        *self
    }
}
