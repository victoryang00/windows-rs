#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
    pub fn InitializeXamlDiagnostic(endpointname: ::windows_sys::core::PCWSTR, pid: u32, wszdllxamldiagnostics: ::windows_sys::core::PCWSTR, wsztapdllname: ::windows_sys::core::PCWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
    pub fn InitializeXamlDiagnosticsEx(endpointname: ::windows_sys::core::PCWSTR, pid: u32, wszdllxamldiagnostics: ::windows_sys::core::PCWSTR, wsztapdllname: ::windows_sys::core::PCWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type BaseValueSource = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceUnknown: BaseValueSource = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceDefault: BaseValueSource = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceStyle: BaseValueSource = 3i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceLocal: BaseValueSource = 4i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Inherited: BaseValueSource = 5i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const DefaultStyleTrigger: BaseValueSource = 6i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const TemplateTrigger: BaseValueSource = 7i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const StyleTrigger: BaseValueSource = 8i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ImplicitStyleReference: BaseValueSource = 9i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ParentTemplate: BaseValueSource = 10i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ParentTemplateTrigger: BaseValueSource = 11i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Animation: BaseValueSource = 12i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Coercion: BaseValueSource = 13i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceVisualState: BaseValueSource = 14i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub AlphaMode: super::super::super::Graphics::Dxgi::Common::DXGI_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for BitmapDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub MetadataBits: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CollectionElementValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CollectionElementValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = -2144665560i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct EnumType {
    pub Name: super::super::super::Foundation::BSTR,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for EnumType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for EnumType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IBitmapData {
    pub base__: ::windows_sys::core::IUnknown,
    pub CopyBytesTo: unsafe extern "system" fn(this: *mut *mut Self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStride: unsafe extern "system" fn(this: *mut *mut Self, pstride: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetBitmapDescription: unsafe extern "system" fn(this: *mut *mut Self, pbitmapdescription: *mut BitmapDescription) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetBitmapDescription: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetSourceBitmapDescription: unsafe extern "system" fn(this: *mut *mut Self, pbitmapdescription: *mut BitmapDescription) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetSourceBitmapDescription: usize,
}
impl ::windows_sys::core::Interface for IBitmapData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3517140722, data2: 51928, data3: 17973, data4: [163, 210, 252, 218, 141, 63, 60, 175] };
}
#[repr(C)]
pub struct IVisualTreeService {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseVisualTreeChange: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnadviseVisualTreeChange: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetEnums: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetEnums: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyValuesChain: unsafe extern "system" fn(this: *mut *mut Self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyValuesChain: usize,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows_sys::core::HRESULT,
    pub ClearProperty: unsafe extern "system" fn(this: *mut *mut Self, instancehandle: u64, propertyindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetCollectionCount: unsafe extern "system" fn(this: *mut *mut Self, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCollectionElements: unsafe extern "system" fn(this: *mut *mut Self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCollectionElements: usize,
    pub AddChild: unsafe extern "system" fn(this: *mut *mut Self, parent: u64, child: u64, index: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut *mut Self, parent: u64, index: u32) -> ::windows_sys::core::HRESULT,
    pub ClearChildren: unsafe extern "system" fn(this: *mut *mut Self, parent: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualTreeService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2777919770, data2: 53631, data3: 18619, data4: [143, 102, 131, 145, 7, 49, 200, 165] };
}
#[repr(C)]
pub struct IVisualTreeService2 {
    pub base__: IVisualTreeService,
    pub GetPropertyIndex: unsafe extern "system" fn(this: *mut *mut Self, object: u64, propertyname: ::windows_sys::core::PCWSTR, ppropertyindex: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ReplaceResource: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows_sys::core::HRESULT,
    pub RenderTargetBitmap: unsafe extern "system" fn(this: *mut *mut Self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualTreeService2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 319770934, data2: 60483, data3: 20321, data4: [137, 199, 152, 1, 163, 109, 46, 149] };
}
#[repr(C)]
pub struct IVisualTreeService3 {
    pub base__: IVisualTreeService2,
    pub ResolveResource: unsafe extern "system" fn(this: *mut *mut Self, resourcecontext: u64, resourcename: ::windows_sys::core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDictionaryItem: unsafe extern "system" fn(this: *mut *mut Self, dictionaryhandle: u64, resourcename: ::windows_sys::core::PCWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDictionaryItem: usize,
    pub AddDictionaryItem: unsafe extern "system" fn(this: *mut *mut Self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows_sys::core::HRESULT,
    pub RemoveDictionaryItem: unsafe extern "system" fn(this: *mut *mut Self, dictionaryhandle: u64, resourcekey: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualTreeService3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242861792, data2: 34208, data3: 19432, data4: [180, 26, 101, 92, 241, 253, 25, 189] };
}
#[repr(C)]
pub struct IVisualTreeServiceCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnVisualTreeChange: unsafe extern "system" fn(this: *mut *mut Self, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnVisualTreeChange: usize,
}
impl ::windows_sys::core::Interface for IVisualTreeServiceCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860157233, data2: 32996, data3: 20460, data4: [143, 59, 85, 63, 135, 180, 150, 110] };
}
#[repr(C)]
pub struct IVisualTreeServiceCallback2 {
    pub base__: IVisualTreeServiceCallback,
    pub OnElementStateChanged: unsafe extern "system" fn(this: *mut *mut Self, element: u64, elementstate: VisualElementState, context: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualTreeServiceCallback2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134843784, data2: 44663, data3: 17303, data4: [185, 72, 95, 162, 219, 10, 25, 234] };
}
#[repr(C)]
pub struct IXamlDiagnostics {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDispatcher: unsafe extern "system" fn(this: *mut *mut Self, ppdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetUiLayer: unsafe extern "system" fn(this: *mut *mut Self, pplayer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(this: *mut *mut Self, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIInspectableFromHandle: unsafe extern "system" fn(this: *mut *mut Self, instancehandle: u64, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHandleFromIInspectable: unsafe extern "system" fn(this: *mut *mut Self, pinstance: *mut ::core::ffi::c_void, phandle: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTest: unsafe extern "system" fn(this: *mut *mut Self, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTest: usize,
    pub RegisterInstance: unsafe extern "system" fn(this: *mut *mut Self, pinstance: *mut ::core::ffi::c_void, pinstancehandle: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInitializationData: unsafe extern "system" fn(this: *mut *mut Self, pinitializationdata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInitializationData: usize,
}
impl ::windows_sys::core::Interface for IXamlDiagnostics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 415883958, data2: 16195, data3: 16662, data4: [159, 43, 255, 147, 93, 119, 112, 210] };
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: Self = Self(0i32);
    pub const IsValueHandle: Self = Self(1i32);
    pub const IsPropertyReadOnly: Self = Self(2i32);
    pub const IsValueCollection: Self = Self(4i32);
    pub const IsValueCollectionReadOnly: Self = Self(8i32);
    pub const IsValueBindingExpression: Self = Self(16i32);
    pub const IsValueNull: Self = Self(32i32);
    pub const IsValueHandleAndEvaluatedValue: Self = Self(64i32);
}
impl ::core::marker::Copy for MetadataBit {}
impl ::core::clone::Clone for MetadataBit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
impl ::core::marker::Copy for ParentChildRelation {}
impl ::core::clone::Clone for ParentChildRelation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainSource {
    pub Handle: u64,
    pub TargetType: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PropertyChainSource {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: super::super::super::Foundation::BSTR,
    pub DeclaringType: super::super::super::Foundation::BSTR,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub ItemType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub Overridden: super::super::super::Foundation::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: super::super::super::Foundation::BSTR,
    pub PropertyChainIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PropertyChainValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type RenderTargetBitmapOptions = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const RenderTarget: RenderTargetBitmapOptions = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type ResourceType = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ResourceTypeStatic: ResourceType = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ResourceTypeTheme: ResourceType = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SourceInfo {
    pub FileName: super::super::super::Foundation::BSTR,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SourceInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SourceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub NumChildren: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VisualElement {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VisualElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type VisualElementState = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorResolved: VisualElementState = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorResourceNotFound: VisualElementState = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorInvalidResource: VisualElementState = 2i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type VisualMutationType = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Add: VisualMutationType = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Remove: VisualMutationType = 1i32;
