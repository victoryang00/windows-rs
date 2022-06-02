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
#[repr(C)]
pub struct IPrintTicketValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintTicketValueType) -> ::windows_sys::core::HRESULT,
    pub GetValueAsInteger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetValueAsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IWorkflowPrintTicketValidationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Validated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
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
