pub type Binding = *mut ::core::ffi::c_void;
pub type BindingBase = *mut ::core::ffi::c_void;
pub type BindingExpression = *mut ::core::ffi::c_void;
pub type BindingExpressionBase = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingMode(pub i32);
impl BindingMode {
    pub const OneWay: Self = Self(1i32);
    pub const OneTime: Self = Self(2i32);
    pub const TwoWay: Self = Self(3i32);
}
impl ::core::marker::Copy for BindingMode {}
impl ::core::clone::Clone for BindingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BindingOperations = *mut ::core::ffi::c_void;
pub type CollectionViewSource = *mut ::core::ffi::c_void;
pub type CurrentChangingEventArgs = *mut ::core::ffi::c_void;
pub type CurrentChangingEventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBinding {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BindingMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: BindingMode) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RelativeSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRelativeSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetElementName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Converter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConverter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConverterParameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConverterParameter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConverterLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetConverterLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBinding2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FallbackValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFallbackValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetNullValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetNullValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateSourceTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UpdateSourceTrigger) -> ::windows_sys::core::HRESULT,
    pub SetUpdateSourceTrigger: unsafe extern "system" fn(this: *mut *mut Self, value: UpdateSourceTrigger) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindingBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBindingBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindingExpression {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ParentBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindingExpressionBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBindingExpressionBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBindingExpressionFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBindingFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindingOperations {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBindingOperationsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBinding: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, dp: *mut ::core::ffi::c_void, binding: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICollectionView {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CurrentPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsCurrentAfterLast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCurrentBeforeFirst: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CollectionGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CollectionGroups: usize,
    pub HasMoreItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CurrentChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanging: usize,
    pub MoveCurrentTo: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveCurrentToPosition: unsafe extern "system" fn(this: *mut *mut Self, index: i32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveCurrentToFirst: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveCurrentToLast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveCurrentToNext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveCurrentToPrevious: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadMoreItemsAsync: usize,
}
#[repr(C)]
pub struct ICollectionViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICollectionViewGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GroupItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GroupItems: usize,
}
#[repr(C)]
pub struct ICollectionViewSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub View: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSourceGrouped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSourceGrouped: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ItemsPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICollectionViewSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSourceGroupedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsPathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICurrentChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCancelable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICurrentChangingEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithCancelableParameter: unsafe extern "system" fn(this: *mut *mut Self, iscancelable: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomProperty {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Type: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIndexedValue: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIndexedValue: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomPropertyProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCustomProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub GetIndexedProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    GetIndexedProperty: usize,
    pub GetStringRepresentation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Type: usize,
}
#[repr(C)]
pub struct IItemIndexRange {
    pub base__: ::windows_sys::core::IInspectable,
    pub FirstIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LastIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemIndexRangeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, firstindex: i32, length: u32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsRangeInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RangesChanged: unsafe extern "system" fn(this: *mut *mut Self, visiblerange: *mut ::core::ffi::c_void, trackeditems: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RangesChanged: usize,
}
#[repr(C)]
pub struct INotifyPropertyChanged {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertyChanged: usize,
}
#[repr(C)]
pub struct IPropertyChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PropertyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPropertyChangedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativeSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RelativeSourceMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: RelativeSourceMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRelativeSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectRange: unsafe extern "system" fn(this: *mut *mut Self, itemindexrange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeselectRange: unsafe extern "system" fn(this: *mut *mut Self, itemindexrange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, index: i32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSelectedRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSelectedRanges: usize,
}
#[repr(C)]
pub struct ISupportIncrementalLoading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadMoreItemsAsync: usize,
    pub HasMoreItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IValueConverter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Convert: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Convert: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub ConvertBack: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    ConvertBack: usize,
}
pub type ItemIndexRange = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
pub struct LoadMoreItemsResult {
    pub Count: u32,
}
impl ::core::marker::Copy for LoadMoreItemsResult {}
impl ::core::clone::Clone for LoadMoreItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PropertyChangedEventArgs = *mut ::core::ffi::c_void;
pub type PropertyChangedEventHandler = *mut ::core::ffi::c_void;
pub type RelativeSource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct RelativeSourceMode(pub i32);
impl RelativeSourceMode {
    pub const None: Self = Self(0i32);
    pub const TemplatedParent: Self = Self(1i32);
    pub const Self_: Self = Self(2i32);
}
impl ::core::marker::Copy for RelativeSourceMode {}
impl ::core::clone::Clone for RelativeSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct UpdateSourceTrigger(pub i32);
impl UpdateSourceTrigger {
    pub const Default: Self = Self(0i32);
    pub const PropertyChanged: Self = Self(1i32);
    pub const Explicit: Self = Self(2i32);
    pub const LostFocus: Self = Self(3i32);
}
impl ::core::marker::Copy for UpdateSourceTrigger {}
impl ::core::clone::Clone for UpdateSourceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
