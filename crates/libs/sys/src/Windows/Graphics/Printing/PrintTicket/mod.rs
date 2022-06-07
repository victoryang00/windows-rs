#[repr(C)]
pub struct IPrintTicketCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DocumentBindingFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetParameterDefinition: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTicketCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2353352843, data2: 48092, data3: 16982, data4: [161, 66, 47, 214, 21, 236, 180, 22] };
}
#[repr(C)]
pub struct IPrintTicketFeature {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetOption: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub GetSelectedOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedOption: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintTicketFeatureSelectionType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTicketFeature {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881860458, data2: 23029, data3: 16643, data4: [136, 88, 185, 119, 16, 150, 61, 57] };
}
#[repr(C)]
pub struct IPrintTicketOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetPropertyNode: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetPropertyNode: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetScoredPropertyNode: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetScoredPropertyNode: usize,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetScoredPropertyValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTicketOption {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2961624976, data2: 45927, data3: 20043, data4: [189, 72, 156, 120, 160, 187, 49, 206] };
}
#[repr(C)]
pub struct IPrintTicketParameterDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintTicketParameterDataType) -> ::windows_sys::core::HRESULT,
    pub UnitType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RangeMin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RangeMax: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTicketParameterDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3602560228, data2: 10594, data3: 19457, data4: [183, 243, 154, 146, 148, 235, 131, 53] };
}
#[repr(C)]
pub struct IPrintTicketParameterInitializer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTicketParameterInitializer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1580414395, data2: 41125, data3: 18609, data4: [157, 92, 7, 17, 109, 220, 89, 122] };
}
#[repr(C)]
pub struct IPrintTicketValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintTicketValueType) -> ::windows_sys::core::HRESULT,
    pub GetValueAsInteger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetValueAsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTicketValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1723009586, data2: 9293, data3: 20002, data4: [169, 139, 187, 60, 241, 242, 221, 145] };
}
#[repr(C)]
pub struct IWorkflowPrintTicket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentBindingFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotifyXmlChangedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyXmlChangedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValidateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateAsync: usize,
    pub GetParameterInitializer: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetParameterInitializerAsInteger: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, integervalue: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetParameterInitializerAsString: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, xmlnamespace: ::windows_sys::core::HSTRING, stringvalue: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MergeAndValidateTicket: unsafe extern "system" fn(this: *mut *mut Self, deltashematicket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkflowPrintTicket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1104487045, data2: 13800, data3: 17550, data4: [168, 197, 228, 182, 162, 207, 130, 108] };
}
#[repr(C)]
pub struct IWorkflowPrintTicketValidationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Validated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkflowPrintTicketValidationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 181531538, data2: 55931, data3: 18998, data4: [191, 54, 106, 153, 166, 46, 32, 89] };
}
pub type PrintTicketCapabilities = *mut ::core::ffi::c_void;
pub type PrintTicketFeature = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketFeatureSelectionType(pub i32);
impl PrintTicketFeatureSelectionType {
    pub const PickOne: Self = Self(0i32);
    pub const PickMany: Self = Self(1i32);
}
impl ::core::marker::Copy for PrintTicketFeatureSelectionType {}
impl ::core::clone::Clone for PrintTicketFeatureSelectionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintTicketOption = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketParameterDataType(pub i32);
impl PrintTicketParameterDataType {
    pub const Integer: Self = Self(0i32);
    pub const NumericString: Self = Self(1i32);
    pub const String: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintTicketParameterDataType {}
impl ::core::clone::Clone for PrintTicketParameterDataType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintTicketParameterDefinition = *mut ::core::ffi::c_void;
pub type PrintTicketParameterInitializer = *mut ::core::ffi::c_void;
pub type PrintTicketValue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketValueType(pub i32);
impl PrintTicketValueType {
    pub const Integer: Self = Self(0i32);
    pub const String: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintTicketValueType {}
impl ::core::clone::Clone for PrintTicketValueType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WorkflowPrintTicket = *mut ::core::ffi::c_void;
pub type WorkflowPrintTicketValidationResult = *mut ::core::ffi::c_void;
