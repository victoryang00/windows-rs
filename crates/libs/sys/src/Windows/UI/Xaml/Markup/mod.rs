#[repr(C)]
pub struct IComponentConnector {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, connectionid: i32, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComponentConnector2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetBindingConnector: unsafe extern "system" fn(this: *mut *mut Self, connectionid: i32, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateComponent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Recycle: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ProcessBindings: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, itemindex: i32, phase: i32, nextphase: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMarkupExtension {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMarkupExtensionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMarkupExtensionOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProvideValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlBinaryWriter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IXamlBinaryWriterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, inputstreams: *mut ::core::ffi::c_void, outputstreams: *mut ::core::ffi::c_void, xamlmetadataprovider: *mut ::core::ffi::c_void, result__: *mut XamlBinaryWriterErrorInformation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Write: usize,
}
#[repr(C)]
pub struct IXamlBindScopeDiagnostics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self, linenumber: i32, columnnumber: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlBindingHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IXamlBindingHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataTemplateComponentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDataTemplateComponent: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDataTemplateComponent: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SuspendRendering: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResumeRendering: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub ConvertValue: unsafe extern "system" fn(this: *mut *mut Self, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    ConvertValue: usize,
    pub SetPropertyFromString: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromBoolean: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromChar16: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromDateTime: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromDateTime: usize,
    pub SetPropertyFromDouble: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: f64) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromInt32: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromUInt32: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: u32) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromInt64: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: i64) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromUInt64: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: u64) -> ::windows_sys::core::HRESULT,
    pub SetPropertyFromSingle: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromPoint: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromRect: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromSize: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromTimeSpan: usize,
    pub SetPropertyFromByte: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromUri: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromUri: usize,
    pub SetPropertyFromObject: unsafe extern "system" fn(this: *mut *mut Self, dependencyobject: *mut ::core::ffi::c_void, propertytoset: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlMarkupHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IXamlMarkupHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnloadObject: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlMember {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAttachable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDependencyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TargetType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, instance: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlMetadataProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub GetXamlType: unsafe extern "system" fn(this: *mut *mut Self, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    GetXamlType: usize,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(this: *mut *mut Self, fullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlReader {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IXamlReaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, xaml: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadWithInitialTemplateValidation: unsafe extern "system" fn(this: *mut *mut Self, xaml: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlType {
    pub base__: ::windows_sys::core::IInspectable,
    pub BaseType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsConstructible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDictionary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMarkupExtension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBindable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ItemType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub UnderlyingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    UnderlyingType: usize,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMember: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(this: *mut *mut Self, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(this: *mut *mut Self, instance: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RunInitializer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlType2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BoxedType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type MarkupExtension = *mut ::core::ffi::c_void;
pub type XamlBinaryWriter = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl ::core::marker::Copy for XamlBinaryWriterErrorInformation {}
impl ::core::clone::Clone for XamlBinaryWriterErrorInformation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XamlBindingHelper = *mut ::core::ffi::c_void;
pub type XamlMarkupHelper = *mut ::core::ffi::c_void;
pub type XamlReader = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows_sys::core::HSTRING,
    pub Namespace: ::windows_sys::core::HSTRING,
}
impl ::core::marker::Copy for XmlnsDefinition {}
impl ::core::clone::Clone for XmlnsDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
