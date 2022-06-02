#[repr(C)]
pub struct IAnnotationProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnnotationTypeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AnnotationTypeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomNavigationProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub NavigateCustom: unsafe extern "system" fn(this: *mut *mut Self, direction: super::Peers::AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    NavigateCustom: usize,
}
#[repr(C)]
pub struct IDockProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub DockPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DockPosition) -> ::windows_sys::core::HRESULT,
    pub SetDockPosition: unsafe extern "system" fn(this: *mut *mut Self, dockposition: super::DockPosition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsGrabbed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DropEffect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DropEffects: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetGrabbedItems: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropTargetProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub DropEffect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DropEffects: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExpandCollapseProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpandCollapseState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ExpandCollapseState) -> ::windows_sys::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Expand: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridItemProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Column: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ColumnSpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ContainingGrid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Row: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RowSpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColumnCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RowCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIRawElementProviderSimple {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInvokeProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemContainerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindItemByProperty: unsafe extern "system" fn(this: *mut *mut Self, startafter: *mut ::core::ffi::c_void, automationproperty: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMultipleViewProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetSupportedViews: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetViewName: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectModelProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRangeValueProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Maximum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Minimum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
pub type IRawElementProviderSimple = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IScrollItemProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontallyScrollable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalScrollPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalViewSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticallyScrollable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub VerticalScrollPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalViewSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Scroll: unsafe extern "system" fn(this: *mut *mut Self, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows_sys::core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(this: *mut *mut Self, horizontalpercent: f64, verticalpercent: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectionItemProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SelectionContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Select: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectionProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanSelectMultiple: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSelectionRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpreadsheetItemProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Formula: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAnnotationObjects: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAnnotationTypes: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut super::AnnotationType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpreadsheetProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetItemByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStylesProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FillColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub FillPatternColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub FillPatternStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Shape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StyleId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StyleName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISynchronizedInputProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartListening: unsafe extern "system" fn(this: *mut *mut Self, inputtype: super::SynchronizedInputType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITableItemProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetColumnHeaderItems: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRowHeaderItems: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITableProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub RowOrColumnMajor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::RowOrColumnMajor) -> ::windows_sys::core::HRESULT,
    pub GetColumnHeaders: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRowHeaders: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextChildProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextEditProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetActiveComposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub DocumentRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::SupportedTextSelection) -> ::windows_sys::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RangeFromChild: unsafe extern "system" fn(this: *mut *mut Self, childelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RangeFromPoint: unsafe extern "system" fn(this: *mut *mut Self, screenlocation: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RangeFromPoint: usize,
}
#[repr(C)]
pub struct ITextProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RangeFromAnnotation: unsafe extern "system" fn(this: *mut *mut Self, annotationelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(this: *mut *mut Self, isactive: *mut bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextRangeProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, textrangeprovider: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub CompareEndpoints: unsafe extern "system" fn(this: *mut *mut Self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: *mut ::core::ffi::c_void, targetendpoint: super::Text::TextPatternRangeEndpoint, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    CompareEndpoints: usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(this: *mut *mut Self, unit: super::Text::TextUnit) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    ExpandToEnclosingUnit: usize,
    pub FindAttribute: unsafe extern "system" fn(this: *mut *mut Self, attributeid: i32, value: *mut ::core::ffi::c_void, backward: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindText: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, backward: bool, ignorecase: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(this: *mut *mut Self, attributeid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBoundingRectangles: unsafe extern "system" fn(this: *mut *mut Self, returnValue_array_size: *mut u32, returnvalue: *mut *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetEnclosingElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, maxlength: i32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    Move: usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub MoveEndpointByUnit: unsafe extern "system" fn(this: *mut *mut Self, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    MoveEndpointByUnit: usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub MoveEndpointByRange: unsafe extern "system" fn(this: *mut *mut Self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: *mut ::core::ffi::c_void, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    MoveEndpointByRange: usize,
    pub Select: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut *mut Self, aligntotop: bool) -> ::windows_sys::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextRangeProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToggleState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ToggleState) -> ::windows_sys::core::HRESULT,
    pub Toggle: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransformProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanMove: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanResize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanRotate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, x: f64, y: f64) -> ::windows_sys::core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, width: f64, height: f64) -> ::windows_sys::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut *mut Self, degrees: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransformProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanZoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxZoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinZoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Zoom: unsafe extern "system" fn(this: *mut *mut Self, zoom: f64) -> ::windows_sys::core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(this: *mut *mut Self, zoomunit: super::ZoomUnit) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IValueProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualizedItemProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Realize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsModal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTopmost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Maximizable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Minimizable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub InteractionState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::WindowInteractionState) -> ::windows_sys::core::HRESULT,
    pub VisualState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::WindowVisualState) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetVisualState: unsafe extern "system" fn(this: *mut *mut Self, state: super::WindowVisualState) -> ::windows_sys::core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(this: *mut *mut Self, milliseconds: i32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
