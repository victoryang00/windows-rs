#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AccessibilityView(pub i32);
impl AccessibilityView {
    pub const Raw: Self = Self(0i32);
    pub const Control: Self = Self(1i32);
    pub const Content: Self = Self(2i32);
}
impl ::core::marker::Copy for AccessibilityView {}
impl ::core::clone::Clone for AccessibilityView {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBarAutomationPeer = *mut ::core::ffi::c_void;
pub type AppBarButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type AppBarToggleButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type AutoSuggestBoxAutomationPeer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationControlType(pub i32);
impl AutomationControlType {
    pub const Button: Self = Self(0i32);
    pub const Calendar: Self = Self(1i32);
    pub const CheckBox: Self = Self(2i32);
    pub const ComboBox: Self = Self(3i32);
    pub const Edit: Self = Self(4i32);
    pub const Hyperlink: Self = Self(5i32);
    pub const Image: Self = Self(6i32);
    pub const ListItem: Self = Self(7i32);
    pub const List: Self = Self(8i32);
    pub const Menu: Self = Self(9i32);
    pub const MenuBar: Self = Self(10i32);
    pub const MenuItem: Self = Self(11i32);
    pub const ProgressBar: Self = Self(12i32);
    pub const RadioButton: Self = Self(13i32);
    pub const ScrollBar: Self = Self(14i32);
    pub const Slider: Self = Self(15i32);
    pub const Spinner: Self = Self(16i32);
    pub const StatusBar: Self = Self(17i32);
    pub const Tab: Self = Self(18i32);
    pub const TabItem: Self = Self(19i32);
    pub const Text: Self = Self(20i32);
    pub const ToolBar: Self = Self(21i32);
    pub const ToolTip: Self = Self(22i32);
    pub const Tree: Self = Self(23i32);
    pub const TreeItem: Self = Self(24i32);
    pub const Custom: Self = Self(25i32);
    pub const Group: Self = Self(26i32);
    pub const Thumb: Self = Self(27i32);
    pub const DataGrid: Self = Self(28i32);
    pub const DataItem: Self = Self(29i32);
    pub const Document: Self = Self(30i32);
    pub const SplitButton: Self = Self(31i32);
    pub const Window: Self = Self(32i32);
    pub const Pane: Self = Self(33i32);
    pub const Header: Self = Self(34i32);
    pub const HeaderItem: Self = Self(35i32);
    pub const Table: Self = Self(36i32);
    pub const TitleBar: Self = Self(37i32);
    pub const Separator: Self = Self(38i32);
    pub const SemanticZoom: Self = Self(39i32);
    pub const AppBar: Self = Self(40i32);
}
impl ::core::marker::Copy for AutomationControlType {}
impl ::core::clone::Clone for AutomationControlType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationEvents(pub i32);
impl AutomationEvents {
    pub const ToolTipOpened: Self = Self(0i32);
    pub const ToolTipClosed: Self = Self(1i32);
    pub const MenuOpened: Self = Self(2i32);
    pub const MenuClosed: Self = Self(3i32);
    pub const AutomationFocusChanged: Self = Self(4i32);
    pub const InvokePatternOnInvoked: Self = Self(5i32);
    pub const SelectionItemPatternOnElementAddedToSelection: Self = Self(6i32);
    pub const SelectionItemPatternOnElementRemovedFromSelection: Self = Self(7i32);
    pub const SelectionItemPatternOnElementSelected: Self = Self(8i32);
    pub const SelectionPatternOnInvalidated: Self = Self(9i32);
    pub const TextPatternOnTextSelectionChanged: Self = Self(10i32);
    pub const TextPatternOnTextChanged: Self = Self(11i32);
    pub const AsyncContentLoaded: Self = Self(12i32);
    pub const PropertyChanged: Self = Self(13i32);
    pub const StructureChanged: Self = Self(14i32);
    pub const DragStart: Self = Self(15i32);
    pub const DragCancel: Self = Self(16i32);
    pub const DragComplete: Self = Self(17i32);
    pub const DragEnter: Self = Self(18i32);
    pub const DragLeave: Self = Self(19i32);
    pub const Dropped: Self = Self(20i32);
    pub const LiveRegionChanged: Self = Self(21i32);
    pub const InputReachedTarget: Self = Self(22i32);
    pub const InputReachedOtherElement: Self = Self(23i32);
    pub const InputDiscarded: Self = Self(24i32);
    pub const WindowClosed: Self = Self(25i32);
    pub const WindowOpened: Self = Self(26i32);
    pub const ConversionTargetChanged: Self = Self(27i32);
    pub const TextEditTextChanged: Self = Self(28i32);
    pub const LayoutInvalidated: Self = Self(29i32);
}
impl ::core::marker::Copy for AutomationEvents {}
impl ::core::clone::Clone for AutomationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationHeadingLevel(pub i32);
impl AutomationHeadingLevel {
    pub const None: Self = Self(0i32);
    pub const Level1: Self = Self(1i32);
    pub const Level2: Self = Self(2i32);
    pub const Level3: Self = Self(3i32);
    pub const Level4: Self = Self(4i32);
    pub const Level5: Self = Self(5i32);
    pub const Level6: Self = Self(6i32);
    pub const Level7: Self = Self(7i32);
    pub const Level8: Self = Self(8i32);
    pub const Level9: Self = Self(9i32);
}
impl ::core::marker::Copy for AutomationHeadingLevel {}
impl ::core::clone::Clone for AutomationHeadingLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationLandmarkType(pub i32);
impl AutomationLandmarkType {
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Form: Self = Self(2i32);
    pub const Main: Self = Self(3i32);
    pub const Navigation: Self = Self(4i32);
    pub const Search: Self = Self(5i32);
}
impl ::core::marker::Copy for AutomationLandmarkType {}
impl ::core::clone::Clone for AutomationLandmarkType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationLiveSetting(pub i32);
impl AutomationLiveSetting {
    pub const Off: Self = Self(0i32);
    pub const Polite: Self = Self(1i32);
    pub const Assertive: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationLiveSetting {}
impl ::core::clone::Clone for AutomationLiveSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationNavigationDirection(pub i32);
impl AutomationNavigationDirection {
    pub const Parent: Self = Self(0i32);
    pub const NextSibling: Self = Self(1i32);
    pub const PreviousSibling: Self = Self(2i32);
    pub const FirstChild: Self = Self(3i32);
    pub const LastChild: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationNavigationDirection {}
impl ::core::clone::Clone for AutomationNavigationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationNotificationKind(pub i32);
impl AutomationNotificationKind {
    pub const ItemAdded: Self = Self(0i32);
    pub const ItemRemoved: Self = Self(1i32);
    pub const ActionCompleted: Self = Self(2i32);
    pub const ActionAborted: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationNotificationKind {}
impl ::core::clone::Clone for AutomationNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationNotificationProcessing(pub i32);
impl AutomationNotificationProcessing {
    pub const ImportantAll: Self = Self(0i32);
    pub const ImportantMostRecent: Self = Self(1i32);
    pub const All: Self = Self(2i32);
    pub const MostRecent: Self = Self(3i32);
    pub const CurrentThenMostRecent: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationNotificationProcessing {}
impl ::core::clone::Clone for AutomationNotificationProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationOrientation(pub i32);
impl AutomationOrientation {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationOrientation {}
impl ::core::clone::Clone for AutomationOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutomationPeer = *mut ::core::ffi::c_void;
pub type AutomationPeerAnnotation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct AutomationStructureChangeType(pub i32);
impl AutomationStructureChangeType {
    pub const ChildAdded: Self = Self(0i32);
    pub const ChildRemoved: Self = Self(1i32);
    pub const ChildrenInvalidated: Self = Self(2i32);
    pub const ChildrenBulkAdded: Self = Self(3i32);
    pub const ChildrenBulkRemoved: Self = Self(4i32);
    pub const ChildrenReordered: Self = Self(5i32);
}
impl ::core::marker::Copy for AutomationStructureChangeType {}
impl ::core::clone::Clone for AutomationStructureChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type ButtonBaseAutomationPeer = *mut ::core::ffi::c_void;
pub type CalendarDatePickerAutomationPeer = *mut ::core::ffi::c_void;
pub type CaptureElementAutomationPeer = *mut ::core::ffi::c_void;
pub type CheckBoxAutomationPeer = *mut ::core::ffi::c_void;
pub type ColorPickerSliderAutomationPeer = *mut ::core::ffi::c_void;
pub type ColorSpectrumAutomationPeer = *mut ::core::ffi::c_void;
pub type ComboBoxAutomationPeer = *mut ::core::ffi::c_void;
pub type ComboBoxItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ComboBoxItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type DatePickerAutomationPeer = *mut ::core::ffi::c_void;
pub type DatePickerFlyoutPresenterAutomationPeer = *mut ::core::ffi::c_void;
pub type FlipViewAutomationPeer = *mut ::core::ffi::c_void;
pub type FlipViewItemAutomationPeer = *mut ::core::ffi::c_void;
pub type FlipViewItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type FlyoutPresenterAutomationPeer = *mut ::core::ffi::c_void;
pub type FrameworkElementAutomationPeer = *mut ::core::ffi::c_void;
pub type GridViewAutomationPeer = *mut ::core::ffi::c_void;
pub type GridViewHeaderItemAutomationPeer = *mut ::core::ffi::c_void;
pub type GridViewItemAutomationPeer = *mut ::core::ffi::c_void;
pub type GridViewItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type GroupItemAutomationPeer = *mut ::core::ffi::c_void;
pub type HubAutomationPeer = *mut ::core::ffi::c_void;
pub type HubSectionAutomationPeer = *mut ::core::ffi::c_void;
pub type HyperlinkButtonAutomationPeer = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppBarAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppBarAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IAppBarButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppBarButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IAppBarToggleButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppBarToggleButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IAutoSuggestBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAutoSuggestBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
    pub EventsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEventsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPattern: unsafe extern "system" fn(this: *mut *mut Self, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RaiseAutomationEvent: unsafe extern "system" fn(this: *mut *mut Self, eventid: AutomationEvents) -> ::windows_sys::core::HRESULT,
    pub RaisePropertyChangedEvent: unsafe extern "system" fn(this: *mut *mut Self, automationproperty: *mut ::core::ffi::c_void, oldvalue: *mut ::core::ffi::c_void, newvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAcceleratorKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAccessKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAutomationControlType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationControlType) -> ::windows_sys::core::HRESULT,
    pub GetAutomationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetBoundingRectangle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBoundingRectangle: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetChildren: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetChildren: usize,
    pub GetClassName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetClickablePoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetClickablePoint: usize,
    pub GetHelpText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetItemStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetLabeledBy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalizedControlType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationOrientation) -> ::windows_sys::core::HRESULT,
    pub HasKeyboardFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsContentElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsControlElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsKeyboardFocusable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOffscreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPassword: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRequiredForForm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetParent: usize,
    pub InvalidatePeer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetPeerFromPoint: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetPeerFromPoint: usize,
    pub GetLiveSetting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationLiveSetting) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer2 {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAutomationPeer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Navigate: unsafe extern "system" fn(this: *mut *mut Self, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetElementFromPoint: unsafe extern "system" fn(this: *mut *mut Self, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetElementFromPoint: usize,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControlledPeers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControlledPeers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAnnotations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAnnotations: usize,
    pub SetParent: unsafe extern "system" fn(this: *mut *mut Self, peer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RaiseTextEditTextChangedEvent: unsafe extern "system" fn(this: *mut *mut Self, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RaiseTextEditTextChangedEvent: usize,
    pub GetPositionInSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetSizeOfSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RaiseStructureChangedEvent: unsafe extern "system" fn(this: *mut *mut Self, structurechangetype: AutomationStructureChangeType, child: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetLandmarkType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationLandmarkType) -> ::windows_sys::core::HRESULT,
    pub GetLocalizedLandmarkType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPeripheral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDataValidForForm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetFullDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCulture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RaiseNotificationEvent: unsafe extern "system" fn(this: *mut *mut Self, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: ::windows_sys::core::HSTRING, activityid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetHeadingLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationHeadingLevel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeer9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDialog: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerAnnotation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::AnnotationType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: super::AnnotationType) -> ::windows_sys::core::HRESULT,
    pub Peer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPeer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerAnnotationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, r#type: super::AnnotationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithPeerParameter: unsafe extern "system" fn(this: *mut *mut Self, r#type: super::AnnotationType, peer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerAnnotationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PeerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPatternCore: unsafe extern "system" fn(this: *mut *mut Self, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAcceleratorKeyCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAccessKeyCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAutomationControlTypeCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationControlType) -> ::windows_sys::core::HRESULT,
    pub GetAutomationIdCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetBoundingRectangleCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBoundingRectangleCore: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetChildrenCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetChildrenCore: usize,
    pub GetClassNameCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetClickablePointCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetClickablePointCore: usize,
    pub GetHelpTextCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetItemStatusCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetItemTypeCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetLabeledByCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalizedControlTypeCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNameCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetOrientationCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationOrientation) -> ::windows_sys::core::HRESULT,
    pub HasKeyboardFocusCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsContentElementCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsControlElementCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnabledCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsKeyboardFocusableCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOffscreenCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPasswordCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRequiredForFormCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetFocusCore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPeerFromPointCore: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPeerFromPointCore: usize,
    pub GetLiveSettingCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationLiveSetting) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerOverrides2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowContextMenuCore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControlledPeersCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControlledPeersCore: usize,
}
#[repr(C)]
pub struct IAutomationPeerOverrides3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NavigateCore: unsafe extern "system" fn(this: *mut *mut Self, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetElementFromPointCore: unsafe extern "system" fn(this: *mut *mut Self, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetElementFromPointCore: usize,
    pub GetFocusedElementCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAnnotationsCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAnnotationsCore: usize,
    pub GetPositionInSetCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetSizeOfSetCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetLevelCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerOverrides4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetLandmarkTypeCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationLandmarkType) -> ::windows_sys::core::HRESULT,
    pub GetLocalizedLandmarkTypeCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerOverrides5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPeripheralCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDataValidForFormCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetFullDescriptionCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDescribedByCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDescribedByCore: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFlowsToCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFlowsToCore: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFlowsFromCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFlowsFromCore: usize,
}
#[repr(C)]
pub struct IAutomationPeerOverrides6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCultureCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerOverrides8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetHeadingLevelCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationHeadingLevel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerOverrides9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDialogCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerProtected {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Automation_Provider")]
    pub PeerFromProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Provider"))]
    PeerFromProvider: usize,
    #[cfg(feature = "UI_Xaml_Automation_Provider")]
    pub ProviderFromPeer: unsafe extern "system" fn(this: *mut *mut Self, peer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Provider"))]
    ProviderFromPeer: usize,
}
#[repr(C)]
pub struct IAutomationPeerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ListenerExists: unsafe extern "system" fn(this: *mut *mut Self, eventid: AutomationEvents, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPeerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GenerateRawElementProviderRuntimeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RawElementProviderRuntimeId) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IButtonBaseAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IButtonBaseAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ICalendarDatePickerAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICalendarDatePickerAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ICaptureElementAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICaptureElementAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ICheckBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICheckBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IColorPickerSliderAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IColorPickerSliderAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IColorSpectrumAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IColorSpectrumAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IComboBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IComboBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IComboBoxItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IComboBoxItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IComboBoxItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IComboBoxItemDataAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatePickerAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDatePickerAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IDatePickerFlyoutPresenterAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlipViewAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlipViewAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IFlipViewItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlipViewItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IFlipViewItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlipViewItemDataAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutPresenterAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFlyoutPresenterAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IFrameworkElementAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Owner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementAutomationPeerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePeerForElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridViewAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IGridViewHeaderItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridViewHeaderItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IGridViewItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridViewItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IGridViewItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridViewItemDataAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGroupItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGroupItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IHubAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHubAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IHubSectionAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHubSectionAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IHyperlinkButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHyperlinkButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IImageAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IImageAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IInkToolbarAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsControlAutomationPeer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControlAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IItemsControlAutomationPeer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateItemAutomationPeer: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsControlAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IItemsControlAutomationPeerOverrides2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnCreateItemAutomationPeer: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListBoxItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListBoxItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListBoxItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListBoxItemDataAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListPickerFlyoutPresenterAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListViewBaseAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewBaseAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListViewBaseHeaderItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewBaseHeaderItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListViewHeaderItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewHeaderItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListViewItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IListViewItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IListViewItemDataAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoopingSelectorAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILoopingSelectorItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILoopingSelectorItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMapControlAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMediaElementAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMediaElementAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IMediaPlayerElementAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMediaPlayerElementAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IMediaTransportControlsAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMediaTransportControlsAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IMenuBarAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuBarAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct IMenuBarItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuBarItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct IMenuFlyoutItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuFlyoutItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IMenuFlyoutPresenterAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMenuFlyoutPresenterAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct INavigationViewItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationViewItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IPasswordBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPasswordBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IPersonPictureAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPersonPictureAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IPickerFlyoutPresenterAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPivotAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPivotAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IPivotItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPivotItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IPivotItemDataAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPivotItemDataAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProgressBarAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IProgressBarAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IProgressRingAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IProgressRingAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRadioButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRadioButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRangeBaseAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRangeBaseAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRatingControlAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRatingControlAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRepeatButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRepeatButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRichEditBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRichEditBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRichTextBlockAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRichTextBlockAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IRichTextBlockOverflowAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRichTextBlockOverflowAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IScrollBarAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IScrollBarAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IScrollViewerAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IScrollViewerAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ISearchBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISearchBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ISelectorAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISelectorAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ISelectorItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISelectorItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithParentAndItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISemanticZoomAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISemanticZoomAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ISettingsFlyoutAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISettingsFlyoutAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ISliderAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISliderAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ITextBlockAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextBlockAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ITextBoxAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextBoxAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IThumbAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IThumbAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ITimePickerAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITimePickerAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ITimePickerFlyoutPresenterAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToggleButtonAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToggleButtonAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IToggleMenuFlyoutItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToggleMenuFlyoutItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct IToggleSwitchAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IToggleSwitchAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ITreeViewItemAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITreeViewItemAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
#[repr(C)]
pub struct ITreeViewListAutomationPeer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITreeViewListAutomationPeerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub CreateInstanceWithOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    CreateInstanceWithOwner: usize,
}
pub type ImageAutomationPeer = *mut ::core::ffi::c_void;
pub type InkToolbarAutomationPeer = *mut ::core::ffi::c_void;
pub type ItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ItemsControlAutomationPeer = *mut ::core::ffi::c_void;
pub type ListBoxAutomationPeer = *mut ::core::ffi::c_void;
pub type ListBoxItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ListBoxItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type ListPickerFlyoutPresenterAutomationPeer = *mut ::core::ffi::c_void;
pub type ListViewAutomationPeer = *mut ::core::ffi::c_void;
pub type ListViewBaseAutomationPeer = *mut ::core::ffi::c_void;
pub type ListViewBaseHeaderItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ListViewHeaderItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ListViewItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ListViewItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type LoopingSelectorAutomationPeer = *mut ::core::ffi::c_void;
pub type LoopingSelectorItemAutomationPeer = *mut ::core::ffi::c_void;
pub type LoopingSelectorItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type MapControlAutomationPeer = *mut ::core::ffi::c_void;
pub type MediaElementAutomationPeer = *mut ::core::ffi::c_void;
pub type MediaPlayerElementAutomationPeer = *mut ::core::ffi::c_void;
pub type MediaTransportControlsAutomationPeer = *mut ::core::ffi::c_void;
pub type MenuBarAutomationPeer = *mut ::core::ffi::c_void;
pub type MenuBarItemAutomationPeer = *mut ::core::ffi::c_void;
pub type MenuFlyoutItemAutomationPeer = *mut ::core::ffi::c_void;
pub type MenuFlyoutPresenterAutomationPeer = *mut ::core::ffi::c_void;
pub type NavigationViewItemAutomationPeer = *mut ::core::ffi::c_void;
pub type PasswordBoxAutomationPeer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
#[repr(transparent)]
pub struct PatternInterface(pub i32);
impl PatternInterface {
    pub const Invoke: Self = Self(0i32);
    pub const Selection: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
    pub const RangeValue: Self = Self(3i32);
    pub const Scroll: Self = Self(4i32);
    pub const ScrollItem: Self = Self(5i32);
    pub const ExpandCollapse: Self = Self(6i32);
    pub const Grid: Self = Self(7i32);
    pub const GridItem: Self = Self(8i32);
    pub const MultipleView: Self = Self(9i32);
    pub const Window: Self = Self(10i32);
    pub const SelectionItem: Self = Self(11i32);
    pub const Dock: Self = Self(12i32);
    pub const Table: Self = Self(13i32);
    pub const TableItem: Self = Self(14i32);
    pub const Toggle: Self = Self(15i32);
    pub const Transform: Self = Self(16i32);
    pub const Text: Self = Self(17i32);
    pub const ItemContainer: Self = Self(18i32);
    pub const VirtualizedItem: Self = Self(19i32);
    pub const Text2: Self = Self(20i32);
    pub const TextChild: Self = Self(21i32);
    pub const TextRange: Self = Self(22i32);
    pub const Annotation: Self = Self(23i32);
    pub const Drag: Self = Self(24i32);
    pub const DropTarget: Self = Self(25i32);
    pub const ObjectModel: Self = Self(26i32);
    pub const Spreadsheet: Self = Self(27i32);
    pub const SpreadsheetItem: Self = Self(28i32);
    pub const Styles: Self = Self(29i32);
    pub const Transform2: Self = Self(30i32);
    pub const SynchronizedInput: Self = Self(31i32);
    pub const TextEdit: Self = Self(32i32);
    pub const CustomNavigation: Self = Self(33i32);
}
impl ::core::marker::Copy for PatternInterface {}
impl ::core::clone::Clone for PatternInterface {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PersonPictureAutomationPeer = *mut ::core::ffi::c_void;
pub type PickerFlyoutPresenterAutomationPeer = *mut ::core::ffi::c_void;
pub type PivotAutomationPeer = *mut ::core::ffi::c_void;
pub type PivotItemAutomationPeer = *mut ::core::ffi::c_void;
pub type PivotItemDataAutomationPeer = *mut ::core::ffi::c_void;
pub type ProgressBarAutomationPeer = *mut ::core::ffi::c_void;
pub type ProgressRingAutomationPeer = *mut ::core::ffi::c_void;
pub type RadioButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type RangeBaseAutomationPeer = *mut ::core::ffi::c_void;
pub type RatingControlAutomationPeer = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Automation_Peers\"`*"]
pub struct RawElementProviderRuntimeId {
    pub Part1: u32,
    pub Part2: u32,
}
impl ::core::marker::Copy for RawElementProviderRuntimeId {}
impl ::core::clone::Clone for RawElementProviderRuntimeId {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RepeatButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type RichEditBoxAutomationPeer = *mut ::core::ffi::c_void;
pub type RichTextBlockAutomationPeer = *mut ::core::ffi::c_void;
pub type RichTextBlockOverflowAutomationPeer = *mut ::core::ffi::c_void;
pub type ScrollBarAutomationPeer = *mut ::core::ffi::c_void;
pub type ScrollViewerAutomationPeer = *mut ::core::ffi::c_void;
pub type SearchBoxAutomationPeer = *mut ::core::ffi::c_void;
pub type SelectorAutomationPeer = *mut ::core::ffi::c_void;
pub type SelectorItemAutomationPeer = *mut ::core::ffi::c_void;
pub type SemanticZoomAutomationPeer = *mut ::core::ffi::c_void;
pub type SettingsFlyoutAutomationPeer = *mut ::core::ffi::c_void;
pub type SliderAutomationPeer = *mut ::core::ffi::c_void;
pub type TextBlockAutomationPeer = *mut ::core::ffi::c_void;
pub type TextBoxAutomationPeer = *mut ::core::ffi::c_void;
pub type ThumbAutomationPeer = *mut ::core::ffi::c_void;
pub type TimePickerAutomationPeer = *mut ::core::ffi::c_void;
pub type TimePickerFlyoutPresenterAutomationPeer = *mut ::core::ffi::c_void;
pub type ToggleButtonAutomationPeer = *mut ::core::ffi::c_void;
pub type ToggleMenuFlyoutItemAutomationPeer = *mut ::core::ffi::c_void;
pub type ToggleSwitchAutomationPeer = *mut ::core::ffi::c_void;
pub type TreeViewItemAutomationPeer = *mut ::core::ffi::c_void;
pub type TreeViewListAutomationPeer = *mut ::core::ffi::c_void;
