#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub mod Peers;
#[cfg(feature = "UI_Xaml_Automation_Provider")]
pub mod Provider;
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub mod Text;
pub type AnnotationPatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: Self = Self(60000i32);
    pub const SpellingError: Self = Self(60001i32);
    pub const GrammarError: Self = Self(60002i32);
    pub const Comment: Self = Self(60003i32);
    pub const FormulaError: Self = Self(60004i32);
    pub const TrackChanges: Self = Self(60005i32);
    pub const Header: Self = Self(60006i32);
    pub const Footer: Self = Self(60007i32);
    pub const Highlighted: Self = Self(60008i32);
    pub const Endnote: Self = Self(60009i32);
    pub const Footnote: Self = Self(60010i32);
    pub const InsertionChange: Self = Self(60011i32);
    pub const DeletionChange: Self = Self(60012i32);
    pub const MoveChange: Self = Self(60013i32);
    pub const FormatChange: Self = Self(60014i32);
    pub const UnsyncedChange: Self = Self(60015i32);
    pub const EditingLockedChange: Self = Self(60016i32);
    pub const ExternalChange: Self = Self(60017i32);
    pub const ConflictingChange: Self = Self(60018i32);
    pub const Author: Self = Self(60019i32);
    pub const AdvancedProofingIssue: Self = Self(60020i32);
    pub const DataValidationError: Self = Self(60021i32);
    pub const CircularReferenceError: Self = Self(60022i32);
}
impl ::core::marker::Copy for AnnotationType {}
impl ::core::clone::Clone for AnnotationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationActiveEnd {}
impl ::core::clone::Clone for AutomationActiveEnd {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: Self = Self(0i32);
    pub const LasVegasLights: Self = Self(1i32);
    pub const BlinkingBackground: Self = Self(2i32);
    pub const SparkleText: Self = Self(3i32);
    pub const MarchingBlackAnts: Self = Self(4i32);
    pub const MarchingRedAnts: Self = Self(5i32);
    pub const Shimmer: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for AutomationAnimationStyle {}
impl ::core::clone::Clone for AutomationAnimationStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutomationAnnotation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: Self = Self(0i32);
    pub const HollowRoundBullet: Self = Self(1i32);
    pub const FilledRoundBullet: Self = Self(2i32);
    pub const HollowSquareBullet: Self = Self(3i32);
    pub const FilledSquareBullet: Self = Self(4i32);
    pub const DashBullet: Self = Self(5i32);
    pub const Other: Self = Self(6i32);
}
impl ::core::marker::Copy for AutomationBulletStyle {}
impl ::core::clone::Clone for AutomationBulletStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: Self = Self(0i32);
    pub const RTL: Self = Self(1i32);
}
impl ::core::marker::Copy for AutomationCaretBidiMode {}
impl ::core::clone::Clone for AutomationCaretBidiMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: Self = Self(0i32);
    pub const EndOfLine: Self = Self(1i32);
    pub const BeginningOfLine: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationCaretPosition {}
impl ::core::clone::Clone for AutomationCaretPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutomationElementIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
    pub const BottomToTop: Self = Self(2i32);
    pub const Vertical: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationFlowDirections {}
impl ::core::clone::Clone for AutomationFlowDirections {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: Self = Self(0i32);
    pub const Outline: Self = Self(1i32);
    pub const Shadow: Self = Self(2i32);
    pub const Engraved: Self = Self(3i32);
    pub const Embossed: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationOutlineStyles {}
impl ::core::clone::Clone for AutomationOutlineStyles {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutomationProperties = *mut ::core::ffi::c_void;
pub type AutomationProperty = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: Self = Self(70001i32);
    pub const Heading2: Self = Self(70002i32);
    pub const Heading3: Self = Self(70003i32);
    pub const Heading4: Self = Self(70004i32);
    pub const Heading5: Self = Self(70005i32);
    pub const Heading6: Self = Self(70006i32);
    pub const Heading7: Self = Self(70007i32);
    pub const Heading8: Self = Self(70008i32);
    pub const Heading9: Self = Self(70009i32);
    pub const Title: Self = Self(70010i32);
    pub const Subtitle: Self = Self(70011i32);
    pub const Normal: Self = Self(70012i32);
    pub const Emphasis: Self = Self(70013i32);
    pub const Quote: Self = Self(70014i32);
    pub const BulletedList: Self = Self(70015i32);
}
impl ::core::marker::Copy for AutomationStyleId {}
impl ::core::clone::Clone for AutomationStyleId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const WordsOnly: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const Dot: Self = Self(4i32);
    pub const Dash: Self = Self(5i32);
    pub const DashDot: Self = Self(6i32);
    pub const DashDotDot: Self = Self(7i32);
    pub const Wavy: Self = Self(8i32);
    pub const ThickSingle: Self = Self(9i32);
    pub const DoubleWavy: Self = Self(10i32);
    pub const ThickWavy: Self = Self(11i32);
    pub const LongDash: Self = Self(12i32);
    pub const ThickDash: Self = Self(13i32);
    pub const ThickDashDot: Self = Self(14i32);
    pub const ThickDashDotDot: Self = Self(15i32);
    pub const ThickDot: Self = Self(16i32);
    pub const ThickLongDash: Self = Self(17i32);
    pub const Other: Self = Self(18i32);
}
impl ::core::marker::Copy for AutomationTextDecorationLineStyle {}
impl ::core::clone::Clone for AutomationTextDecorationLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: Self = Self(0i32);
    pub const AutoCorrect: Self = Self(1i32);
    pub const Composition: Self = Self(2i32);
    pub const CompositionFinalized: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationTextEditChangeType {}
impl ::core::clone::Clone for AutomationTextEditChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DockPatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Fill: Self = Self(4i32);
    pub const None: Self = Self(5i32);
}
impl ::core::marker::Copy for DockPosition {}
impl ::core::clone::Clone for DockPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DragPatternIdentifiers = *mut ::core::ffi::c_void;
pub type DropTargetPatternIdentifiers = *mut ::core::ffi::c_void;
pub type ExpandCollapsePatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: Self = Self(0i32);
    pub const Expanded: Self = Self(1i32);
    pub const PartiallyExpanded: Self = Self(2i32);
    pub const LeafNode: Self = Self(3i32);
}
impl ::core::marker::Copy for ExpandCollapseState {}
impl ::core::clone::Clone for ExpandCollapseState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GridItemPatternIdentifiers = *mut ::core::ffi::c_void;
pub type GridPatternIdentifiers = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAnnotationPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAnnotationPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnnotationTypeIdProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnnotationTypeNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AuthorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DateTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationAnnotation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnnotationType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: AnnotationType) -> ::windows_sys::core::HRESULT,
    pub Element: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationAnnotationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, r#type: AnnotationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithElementParameter: unsafe extern "system" fn(this: *mut *mut Self, r#type: AnnotationType, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationAnnotationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceleratorKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutomationIdProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BoundingRectangleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClassNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClickablePointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ControlTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasKeyboardFocusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HelpTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsContentElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsControlElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsKeyboardFocusableProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOffscreenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPasswordProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRequiredForFormProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemStatusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LabeledByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalizedControlTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LiveSettingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ControlledPeersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PositionInSetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SizeOfSetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LevelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnnotationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LandmarkTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalizedLandmarkTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPeripheralProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsDataValidForFormProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FullDescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DescribedByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlowsToProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlowsFromProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CultureProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeadingLevelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElementIdentifiersStatics8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDialogProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationProperties {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceleratorKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAcceleratorKey: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAcceleratorKey: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAccessKey: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AutomationIdProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAutomationId: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAutomationId: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HelpTextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHelpText: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHelpText: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsRequiredForFormProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsRequiredForForm: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRequiredForForm: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub ItemStatusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetItemStatus: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetItemStatus: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ItemTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetItemType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LabeledByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLabeledBy: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLabeledBy: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LiveSettingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetLiveSetting: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut Peers::AutomationLiveSetting) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetLiveSetting: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetLiveSetting: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: Peers::AutomationLiveSetting) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetLiveSetting: usize,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccessibilityViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetAccessibilityView: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut Peers::AccessibilityView) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetAccessibilityView: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetAccessibilityView: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: Peers::AccessibilityView) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetAccessibilityView: usize,
    pub ControlledPeersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControlledPeers: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControlledPeers: usize,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PositionInSetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPositionInSet: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPositionInSet: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub SizeOfSetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSizeOfSet: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSizeOfSet: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub LevelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub AnnotationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAnnotations: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAnnotations: usize,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LandmarkTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetLandmarkType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut Peers::AutomationLandmarkType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetLandmarkType: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetLandmarkType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: Peers::AutomationLandmarkType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetLandmarkType: usize,
    pub LocalizedLandmarkTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalizedLandmarkType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocalizedLandmarkType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPeripheralProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsPeripheral: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPeripheral: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDataValidForFormProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsDataValidForForm: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDataValidForForm: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub FullDescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFullDescription: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFullDescription: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalizedControlTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalizedControlType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocalizedControlType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DescribedByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDescribedBy: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDescribedBy: usize,
    pub FlowsToProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFlowsTo: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFlowsTo: usize,
    pub FlowsFromProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFlowsFrom: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFlowsFrom: usize,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CultureProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCulture: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCulture: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeadingLevelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetHeadingLevel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut Peers::AutomationHeadingLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetHeadingLevel: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetHeadingLevel: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: Peers::AutomationHeadingLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetHeadingLevel: usize,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDialogProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsDialog: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDialog: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationPropertiesStatics9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutomationControlTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetAutomationControlType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut Peers::AutomationControlType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetAutomationControlType: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetAutomationControlType: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: Peers::AutomationControlType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetAutomationControlType: usize,
}
#[repr(C)]
pub struct IAutomationProperty {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDockPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDockPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DockPositionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDragPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DropEffectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DropEffectsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GrabbedItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsGrabbedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropTargetPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDropTargetPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DropTargetEffectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DropTargetEffectsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExpandCollapsePatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IExpandCollapsePatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpandCollapseStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridItemPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridItemPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColumnProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColumnSpanProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContainingGridProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowSpanProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColumnCountProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowCountProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMultipleViewPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMultipleViewPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SupportedViewsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRangeValuePatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRangeValuePatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LargeChangeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaximumProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SmallChangeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IScrollPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontallyScrollableProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalScrollPercentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalViewSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NoScroll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticallyScrollableProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalScrollPercentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalViewSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectionItemPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISelectionItemPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelectedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionContainerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectionPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISelectionPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanSelectMultipleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSelectionRequiredProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpreadsheetItemPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISpreadsheetItemPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormulaProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStylesPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IStylesPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedPropertiesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FillColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FillPatternColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FillPatternStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShapeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StyleIdProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StyleNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITableItemPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITableItemPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColumnHeaderItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowHeaderItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITablePatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITablePatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColumnHeadersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowHeadersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RowOrColumnMajorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITogglePatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITogglePatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToggleStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransformPattern2Identifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITransformPattern2IdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanZoomProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomLevelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxZoomProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinZoomProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransformPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITransformPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanMoveProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanResizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanRotateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IValuePatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IValuePatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowPatternIdentifiers {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWindowPatternIdentifiersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanMaximizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanMinimizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsModalProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTopmostProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WindowInteractionStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WindowVisualStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type MultipleViewPatternIdentifiers = *mut ::core::ffi::c_void;
pub type RangeValuePatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: Self = Self(0i32);
    pub const ColumnMajor: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for RowOrColumnMajor {}
impl ::core::clone::Clone for RowOrColumnMajor {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: Self = Self(0i32);
    pub const SmallDecrement: Self = Self(1i32);
    pub const NoAmount: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ScrollAmount {}
impl ::core::clone::Clone for ScrollAmount {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ScrollPatternIdentifiers = *mut ::core::ffi::c_void;
pub type SelectionItemPatternIdentifiers = *mut ::core::ffi::c_void;
pub type SelectionPatternIdentifiers = *mut ::core::ffi::c_void;
pub type SpreadsheetItemPatternIdentifiers = *mut ::core::ffi::c_void;
pub type StylesPatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for SupportedTextSelection {}
impl ::core::clone::Clone for SupportedTextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: Self = Self(1i32);
    pub const KeyDown: Self = Self(2i32);
    pub const LeftMouseUp: Self = Self(4i32);
    pub const LeftMouseDown: Self = Self(8i32);
    pub const RightMouseUp: Self = Self(16i32);
    pub const RightMouseDown: Self = Self(32i32);
}
impl ::core::marker::Copy for SynchronizedInputType {}
impl ::core::clone::Clone for SynchronizedInputType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TableItemPatternIdentifiers = *mut ::core::ffi::c_void;
pub type TablePatternIdentifiers = *mut ::core::ffi::c_void;
pub type TogglePatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for ToggleState {}
impl ::core::clone::Clone for ToggleState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TransformPattern2Identifiers = *mut ::core::ffi::c_void;
pub type TransformPatternIdentifiers = *mut ::core::ffi::c_void;
pub type ValuePatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: Self = Self(0i32);
    pub const Closing: Self = Self(1i32);
    pub const ReadyForUserInteraction: Self = Self(2i32);
    pub const BlockedByModalWindow: Self = Self(3i32);
    pub const NotResponding: Self = Self(4i32);
}
impl ::core::marker::Copy for WindowInteractionState {}
impl ::core::clone::Clone for WindowInteractionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WindowPatternIdentifiers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: Self = Self(0i32);
    pub const Maximized: Self = Self(1i32);
    pub const Minimized: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowVisualState {}
impl ::core::clone::Clone for WindowVisualState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Automation\"`*"]
#[repr(transparent)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: Self = Self(0i32);
    pub const LargeDecrement: Self = Self(1i32);
    pub const SmallDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ZoomUnit {}
impl ::core::clone::Clone for ZoomUnit {
    fn clone(&self) -> Self {
        *self
    }
}
